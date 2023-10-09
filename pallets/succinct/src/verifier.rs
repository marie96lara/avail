use ark_bn254::{Bn254, Fq, Fq2, Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_groth16::{prepare_verifying_key, verify_proof, Proof, VerifyingKey};
use ark_std::str::FromStr;
use ark_std::string::String;
use ark_std::string::ToString;
use ark_std::vec;
use ark_std::vec::Vec;
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use sha2::Sha256;
use sp_core::U256;

use frame_system::Config;

use crate::state::{CircomProof, LightClientRotate, LightClientStep, PublicSignals};
use crate::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode, TypeInfo)]
pub enum VerificationError {
	InvalidProof,
	SyncCommitteeNotInitialized,
	NotEnoughSyncCommitteeParticipants,
	ProofNotValid,
	VerificationError,
}

impl<T: Config> From<VerificationError> for Error<T> {
	fn from(e: VerificationError) -> Error<T> {
		match e {
			VerificationError::InvalidProof => Error::<T>::VerificationError,
			// ContractError::SyncCommitteeNotInitialized => {}
			// ContractError::NotEnoughSyncCommitteeParticipants => {}
			// ContractError::ProofNotValid => {}
			// ContractError::VerificationError => {}
			_ => Error::<T>::VerificationError,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode, TypeInfo)]
pub struct Verifier {
	pub vk_json: VerifyingKeyJson,
}

#[derive(Debug)]
pub enum VKeyDeserializationError {
	SerdeError,
}

impl Verifier {
	/// Creates `Verifier` from json representation
	pub fn from_json_u8_slice(slice: &[u8]) -> Result<Self, VKeyDeserializationError> {
		serde_json::from_slice(slice).map_err(|_| VKeyDeserializationError::SerdeError)
	}

	pub fn verify_proof(
		self,
		proof: Proof<Bn254>,
		inputs: &[Fr],
	) -> Result<bool, VerificationError> {
		let vk = self.vk_json.to_verifying_key();
		let pvk = prepare_verifying_key(&vk);

		let result = verify_proof(&pvk, &proof, inputs);
		result.map_err(|_| VerificationError::InvalidProof)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode, TypeInfo)]
pub struct VerifyingKeyJson {
	#[serde(rename = "IC")]
	pub ic: Vec<Vec<String>>,

	#[serde(rename = "nPublic")]
	pub inputs_count: u32,
	pub vk_alpha_1: Vec<String>,
	pub vk_beta_2: Vec<Vec<String>>,
	pub vk_gamma_2: Vec<Vec<String>>,
	pub vk_delta_2: Vec<Vec<String>>,
	pub vk_alphabeta_12: Vec<Vec<Vec<String>>>,
	pub curve: String,
	pub protocol: String,
}

impl VerifyingKeyJson {
	pub fn to_verifying_key(self) -> VerifyingKey<Bn254> {
		let alpha_g1 = G1Affine::from(G1Projective::new(
			str_to_fq(&self.vk_alpha_1[0]),
			str_to_fq(&self.vk_alpha_1[1]),
			str_to_fq(&self.vk_alpha_1[2]),
		));
		let beta_g2 = G2Affine::from(G2Projective::new(
			// x
			Fq2::new(
				str_to_fq(&self.vk_beta_2[0][0]),
				str_to_fq(&self.vk_beta_2[0][1]),
			),
			// y
			Fq2::new(
				str_to_fq(&self.vk_beta_2[1][0]),
				str_to_fq(&self.vk_beta_2[1][1]),
			),
			// z,
			Fq2::new(
				str_to_fq(&self.vk_beta_2[2][0]),
				str_to_fq(&self.vk_beta_2[2][1]),
			),
		));

		let gamma_g2 = G2Affine::from(G2Projective::new(
			// x
			Fq2::new(
				str_to_fq(&self.vk_gamma_2[0][0]),
				str_to_fq(&self.vk_gamma_2[0][1]),
			),
			// y
			Fq2::new(
				str_to_fq(&self.vk_gamma_2[1][0]),
				str_to_fq(&self.vk_gamma_2[1][1]),
			),
			// z,
			Fq2::new(
				str_to_fq(&self.vk_gamma_2[2][0]),
				str_to_fq(&self.vk_gamma_2[2][1]),
			),
		));

		let delta_g2 = G2Affine::from(G2Projective::new(
			// x
			Fq2::new(
				str_to_fq(&self.vk_delta_2[0][0]),
				str_to_fq(&self.vk_delta_2[0][1]),
			),
			// y
			Fq2::new(
				str_to_fq(&self.vk_delta_2[1][0]),
				str_to_fq(&self.vk_delta_2[1][1]),
			),
			// z,
			Fq2::new(
				str_to_fq(&self.vk_delta_2[2][0]),
				str_to_fq(&self.vk_delta_2[2][1]),
			),
		));

		let gamma_abc_g1: Vec<G1Affine> = self
			.ic
			.iter()
			.map(|coords| {
				G1Affine::from(G1Projective::new(
					str_to_fq(&coords[0]),
					str_to_fq(&coords[1]),
					str_to_fq(&coords[2]),
				))
			})
			.collect();

		VerifyingKey::<Bn254> {
			alpha_g1,
			beta_g2,
			gamma_g2,
			delta_g2,
			gamma_abc_g1,
		}
	}
}

pub fn str_to_fq(s: &str) -> Fq {
	Fq::from_str(s).unwrap()
}

pub fn zk_light_client_rotate(
	update: &LightClientRotate,
	verifier: Verifier,
) -> Result<bool, VerificationError> {
	// TODO inputs count?
	let mut inputs = vec!["0".to_string(); 65];

	let m: &mut [u8; 32] = &mut [0u8; 32];
	update.sync_committee_ssz.to_big_endian(m);

	for i in 0..32 {
		inputs[i] = m[i].to_string();
	}

	let header_root = update.step.finalized_header_root.as_bytes();
	let finalized_header_root_numeric = U256::from_big_endian(header_root);

	let m1: &mut [u8; 32] = &mut [0u8; 32];
	finalized_header_root_numeric.to_big_endian(m1);

	for i in 0..32 {
		inputs[32 + i] = m1[i].to_string();
	}

	inputs[64] = U256::from_little_endian(&update.sync_committee_poseidon.encode()).to_string();

	let groth_16_proof = update.proof.clone();
	let circom_proof = CircomProof::new(groth_16_proof.a, groth_16_proof.b, groth_16_proof.c);
	let proof = circom_proof.to_proof();
	let public_signals = PublicSignals::from(inputs);

	let res = verifier.verify_proof(proof.clone(), &public_signals.get());

	res
}

pub fn zk_light_client_step(
	update: &LightClientStep,
	sync_committee_poseidon: U256,
	verifier: Verifier,
) -> Result<bool, VerificationError> {
	let mut fs: [u8; 32] = [0u8; 32];
	let mut pc: [u8; 32] = [0u8; 32];

	let finalized_slot_le = update.finalized_slot.to_le_bytes();
	let participation_le = update.participation.to_le_bytes();
	fs[..finalized_slot_le.len()].copy_from_slice(&finalized_slot_le);
	pc[..participation_le.len()].copy_from_slice(&participation_le);

	let mut h = [0u8; 32];
	let mut temp = [0u8; 64];
	// sha256 & combine inputs
	temp[..32].copy_from_slice(&fs);
	temp[32..].copy_from_slice(&update.finalized_header_root.as_bytes());
	h.copy_from_slice(&Sha256::digest(temp));

	temp[..32].copy_from_slice(&h);
	temp[32..].copy_from_slice(&pc);
	h.copy_from_slice(&Sha256::digest(temp));

	temp[..32].copy_from_slice(&h);
	temp[32..].copy_from_slice(&update.execution_state_root.as_bytes());
	h.copy_from_slice(&Sha256::digest(temp));

	temp[..32].copy_from_slice(&h);
	temp[32..].copy_from_slice(&sync_committee_poseidon.encode());
	h.copy_from_slice(&Sha256::digest(temp));

	// TODO: Confirm this is the correct math!
	let mut t = [255u8; 32];
	t[31] = 0b00011111;

	for i in 0..32 {
		t[i] &= h[i];
	}

	let inputs_string = U256::from_little_endian(t.as_slice()).to_string();

	let inputs = vec![inputs_string; 1];

	let groth_16_proof = update.proof.clone();

	let circom_proof = CircomProof::new(groth_16_proof.a, groth_16_proof.b, groth_16_proof.c);
	let proof = circom_proof.to_proof();
	let public_signals = PublicSignals::from(inputs);

	verifier.verify_proof(proof, &public_signals.get())
}

#[cfg(test)]
mod tests {
	use ark_std::string::ToString;
	use ark_std::vec;
	use frame_support::assert_ok;
	use hex_literal::hex;
	use sp_core::{H256, U256};

	use crate::state::{Groth16Proof, LightClientRotate, LightClientStep, State};
	use crate::verifier::{zk_light_client_rotate, zk_light_client_step, Verifier};

	#[test]
	fn test_zk_step() {
		let state: State = State {
			updater: H256([0u8; 32]),
			genesis_validators_root: H256([0u8; 32]),
			genesis_time: 1696404557,
			seconds_per_slot: 12000,
			slots_per_period: 8192,
			source_chain_id: 0,
			finality_threshold: 1,
			head: 0,
			consistent: true,
		};

		let finalized_header_root = H256(hex!(
			"70d0a7f53a459dd88eb37c6cfdfb8c48f120e504c96b182357498f2691aa5653"
		));
		let execution_state_root = H256(hex!(
			"69d746cb81cd1fb4c11f4dcc04b6114596859b518614da0dd3b4192ff66c3a58"
		));
		let sync_committee_poseidon = U256::from_dec_str(
			"7032059424740925146199071046477651269705772793323287102921912953216115444414",
		)
		.unwrap();

		let lcs = LightClientStep {
            attested_slot: 0,
            finalized_slot: 4359840,
            participation: 432,
            finalized_header_root,
            execution_state_root,
            proof: Groth16Proof {
                a: vec![
                    "14717729948616455402271823418418032272798439132063966868750456734930753033999"
                        .to_string(),
                    "10284862272179454279380723177303354589165265724768792869172425850641532396958"
                        .to_string(),
                ],
                b: vec![
                    vec![
                        "11269943315518713067124801671029240901063146909738584854987772776806315890545"
                            .to_string(),
                        "20094085308485991030092338753416508135313449543456147939097124612984047201335"
                            .to_string(),
                    ],
                    vec![
                        "8122139689435793554974799663854817979475528090524378333920791336987132768041"
                            .to_string(),
                        "5111528818556913201486596055325815760919897402988418362773344272232635103877"
                            .to_string(),
                    ],
                ],
                c: vec![
                    "6410073677012431469384941862462268198904303371106734783574715889381934207004"
                        .to_string(),
                    "11977981471972649035068934866969447415783144961145315609294880087827694234248"
                        .to_string(),
                ],
            },
        };
		let vk = r#"{"vk_json":{
 "protocol": "groth16",
 "curve": "bn128",
 "nPublic": 1,
 "vk_alpha_1": [
  "20491192805390485299153009773594534940189261866228447918068658471970481763042",
  "9383485363053290200918347156157836566562967994039712273449902621266178545958",
  "1"
 ],
 "vk_beta_2": [
  [
   "6375614351688725206403948262868962793625744043794305715222011528459656738731",
   "4252822878758300859123897981450591353533073413197771768651442665752259397132"
  ],
  [
   "10505242626370262277552901082094356697409835680220590971873171140371331206856",
   "21847035105528745403288232691147584728191162732299865338377159692350059136679"
  ],
  [
   "1",
   "0"
  ]
 ],
 "vk_gamma_2": [
  [
   "10857046999023057135944570762232829481370756359578518086990519993285655852781",
   "11559732032986387107991004021392285783925812861821192530917403151452391805634"
  ],
  [
   "8495653923123431417604973247489272438418190587263600148770280649306958101930",
   "4082367875863433681332203403145435568316851327593401208105741076214120093531"
  ],
  [
   "1",
   "0"
  ]
 ],
 "vk_delta_2": [
  [
   "13909124302531010921185816266702828674819977847946098152869315744616458486564",
   "20132301864891590102651537900097603129841488311097169471951837821863335966377"
  ],
  [
   "9968363667543645393414941586581030294599633785037951467496223618072496422152",
   "19620890790369364323423864638476333921325558259845161848280036523505618212219"
  ],
  [
   "1",
   "0"
  ]
 ],
 "vk_alphabeta_12": [
  [
   [
    "2029413683389138792403550203267699914886160938906632433982220835551125967885",
    "21072700047562757817161031222997517981543347628379360635925549008442030252106"
   ],
   [
    "5940354580057074848093997050200682056184807770593307860589430076672439820312",
    "12156638873931618554171829126792193045421052652279363021382169897324752428276"
   ],
   [
    "7898200236362823042373859371574133993780991612861777490112507062703164551277",
    "7074218545237549455313236346927434013100842096812539264420499035217050630853"
   ]
  ],
  [
   [
    "7077479683546002997211712695946002074877511277312570035766170199895071832130",
    "10093483419865920389913245021038182291233451549023025229112148274109565435465"
   ],
   [
    "4595479056700221319381530156280926371456704509942304414423590385166031118820",
    "19831328484489333784475432780421641293929726139240675179672856274388269393268"
   ],
   [
    "11934129596455521040620786944827826205713621633706285934057045369193958244500",
    "8037395052364110730298837004334506829870972346962140206007064471173334027475"
   ]
  ]
 ],
 "IC": [
  [
   "14768330346746297840816367070658728893313212053739352195802618469166531204391",
   "226007277514949219964518589190903213280753732819328898150443666757283640566",
   "1"
  ],
  [
   "11579789275084599412171695990815953848893751967864880119324773293908098730772",
   "7016524000863123597202679959446996204295974709290664682467334394757983209848",
   "1"
  ]
 ]
}}"#;

		let v = Verifier::from_json_u8_slice(vk.as_bytes()).unwrap();

		assert_eq!("bn128", v.vk_json.curve);
		assert_eq!("groth16", v.vk_json.protocol);
		assert_eq!(
			"9383485363053290200918347156157836566562967994039712273449902621266178545958",
			v.vk_json.vk_alpha_1[1]
		);
		assert_eq!(
			"21847035105528745403288232691147584728191162732299865338377159692350059136679",
			v.vk_json.vk_beta_2[1][1]
		);
		assert_eq!(
			"8495653923123431417604973247489272438418190587263600148770280649306958101930",
			v.vk_json.vk_gamma_2[1][0]
		);

		assert_eq!(
			"2029413683389138792403550203267699914886160938906632433982220835551125967885",
			v.vk_json.vk_alphabeta_12[0][0][0]
		);
		assert_eq!(
			"7016524000863123597202679959446996204295974709290664682467334394757983209848",
			v.vk_json.ic[1][1]
		);

		let res = zk_light_client_step(&lcs, sync_committee_poseidon, v);

		assert_ok!(res.clone());
		assert_eq!(true, res.unwrap());
	}

	#[test]
	fn test_zk_rotate_with_serde() {
		let state: State = State {
			updater: H256::zero(),
			genesis_validators_root: H256::zero(),
			genesis_time: 1696404557,
			seconds_per_slot: 12000,
			slots_per_period: 8192,
			source_chain_id: 0,
			finality_threshold: 1,
			head: 0,
			consistent: true,
		};

		let finalized_header_root = H256(hex!(
			"b6c60352d13b5a1028a99f11ec314004da83c9dbc58b7eba72ae71b3f3373c30"
		));
		let execution_state_root = H256(hex!(
			"ef6dc7ca7a8a7d3ab379fa196b1571398b0eb9744e2f827292c638562090f0cb"
		));
		let sync_committee_poseidon = U256::from_dec_str(
			"13340003662261458565835017692041308090002736850267009725732232370707087749826",
		)
		.unwrap();

		let h = hex!("c1c5193ee38508e60af26d51b83e2c6ba6934fd00d2bb8cb36e95d5402fbfc94");

		let sync_committee_ssz = U256::from_big_endian(h.as_slice());

		let proof = Groth16Proof {
			a: vec![
				"2389393404492058253160068022258603729350770245558596428430133000235269498543"
					.to_string(),
				"10369223312690872346127509312343439494640770569110984786213351208635909948543"
					.to_string(),
			],
			b: vec![
				vec![
					"11815959921059098071620606293769973610509565967606374482200288258603855668773"
						.to_string(),
					"10181085549071219170085204492459257955822340639736743687662735377741773005552"
						.to_string(),
				],
				vec![
					"4596699114942981172597823241348081341260261170814329779716288274614793962155"
						.to_string(),
					"14404189974461708010365785617881368513005872936409632496299813856721680720909"
						.to_string(),
				],
			],
			c: vec![
				"9035222358509333553848504918662877956429157268124015769960938782858405579405"
					.to_string(),
				"10878155942650055578211805190943912843265267774943864267206635407924778282720"
					.to_string(),
			],
		};

		let ssz_proof = Groth16Proof {
			a: vec![
				"19432175986645681540999611667567820365521443728844489852797484819167568900221"
					.to_string(),
				"17819747348018194504213652705429154717568216715442697677977860358267208774881"
					.to_string(),
			],
			b: vec![
				vec![
					"19517979001366784491262985007208187156868482446794264383959847800886523509877"
						.to_string(),
					"18685503971201701637279255177672737459369364286579884138384195256096640826544"
						.to_string(),
				],
				vec![
					"16475201747689810182851523453109345313415173394858409181213088485065940128783"
						.to_string(),
					"12866135194889417072846904485239086915117156987867139218395654387586559304324"
						.to_string(),
				],
			],
			c: vec![
				"5276319441217508855890249255054235161211918914051110197093775833187899960891"
					.to_string(),
				"14386728697935258641600181574898746001129655942955900029040036823246860905307"
					.to_string(),
			],
		};

		let lcs = LightClientRotate {
			step: LightClientStep {
				attested_slot: 0,
				finalized_slot: 4360032,
				participation: 413,
				finalized_header_root,
				execution_state_root,
				proof,
			},

			sync_committee_ssz,
			sync_committee_poseidon,
			proof: ssz_proof,
		};
		let vk = r#"{
  "vk_json": {
    "IC": [
      [
        "8578436021932201623189950508428893454182388340351344018505166330143567388321",
        "16791589135888029668423043760081149899869808657673802901072541038338364024546",
        "1"
      ],
      [
        "25701822399942772837622355873666773966584557696246630986145745837743012328",
        "9106296548924249216714445588642408292162429818431200049858166820587553300743",
        "1"
      ],
      [
        "3887662612813233731242849276337128617088918625889507185164703631018117727457",
        "13896811173741308528708014989807693837248126243365939233014522115172481095858",
        "1"
      ],
      [
        "5548374792924448906382503954713620545264383960034078132831347681208659883879",
        "18337979253751456511617696979491402846813972161597496106673851097218050546456",
        "1"
      ],
      [
        "3513238722103274273522406813717310357132624410145966651049892804577998022250",
        "2100309871897719369243436360393091182026895390103977792742478850790465589269",
        "1"
      ],
      [
        "16774465038985513133855420166203428339905539044232825240202423848107973332678",
        "16725209702452907720642671886251029659390071036535497310393123580872513582854",
        "1"
      ],
      [
        "12723222834246347599931898986533294650712552047289490705825258098822663181624",
        "15583690586388405262832138004749201253661764369810956711623986951827016385530",
        "1"
      ],
      [
        "5026046789904582550128325004633260710365247286992446247401846497370999862747",
        "4819305538846217744687809604109550399579898666648742504258146150555058505951",
        "1"
      ],
      [
        "10661978866672543270025821549099960066183206641640607583795628600555660041044",
        "2221705381270113399691209125100432726126638608264735073397284796557721989416",
        "1"
      ],
      [
        "14569572986724347962216163519300364172959753116406286139655301725097646105213",
        "7770341587360597134866306712713249203225481402263847954302862733679631296332",
        "1"
      ],
      [
        "12051848431286534627025225721668847872601656750023602432136473514335384207905",
        "16398462036535521278977084915557583121149215752915386948180254636830456435800",
        "1"
      ],
      [
        "17788516549705187249706048705237434803985726058264305192488661365056352231811",
        "9833324865779724950328599003907366233834043080911289073114570858263518851267",
        "1"
      ],
      [
        "13791407112521867690666798946890378866880739926855704351528800097495737005313",
        "9534280154764496754119106345378471704562751105412798642818003985309684155151",
        "1"
      ],
      [
        "11248157595270537118518876790672097520286458263406838461943898126202312022003",
        "16525335969593025378236490977703788022845534987776992759419681152792801933150",
        "1"
      ],
      [
        "7642481625425142796479825182895684552997460824738481219118229232519255016122",
        "8250883196634402505522270306684594922079910587826698471154644440215155175065",
        "1"
      ],
      [
        "20425417013275916359632478635869739544705063913976799283081298827713835008110",
        "20477302060644465575718455696912324317560351559288633190546290046579607127782",
        "1"
      ],
      [
        "5425654060952217831080366579755421820472382281006453564161764015857007147308",
        "2950774582227936811007364319920842851514992726670461455045056439315868976302",
        "1"
      ],
      [
        "5345436432687112692120516705320110649658622123412198897333887448750105754094",
        "12300050264583876513872817905211693972346409799324322143353629794626148303442",
        "1"
      ],
      [
        "12973751590041412652384666204593322273486088127424548185584690698646088515366",
        "19439454078430752010862439376052076513472883096982623546597398436535919207925",
        "1"
      ],
      [
        "4051955477657354364034588331948693118288989339139747954526800546287590433367",
        "8422113484412961376284807099601639543331373228327596131273344384708959637800",
        "1"
      ],
      [
        "16034208249619483528537324662673305101108919627658789222516455169942322626890",
        "2345370686898341341833172192052081748607196864441662005601752842695199604273",
        "1"
      ],
      [
        "7162278218711168144539456933597029036462883651507713370483520258091030047429",
        "93445815624105727412582372167300715983431681136591221819840916128859591933",
        "1"
      ],
      [
        "17214438362215194946528883491695011285608827300245652564670983209636706940975",
        "16977413402930132070461984996254832883263273928993725459833681575765046188153",
        "1"
      ],
      [
        "3591514883641765161254179883208720849262342830862751841947669343164678820135",
        "16022324464419654289219272969977271672139780078813781022874177286765631712277",
        "1"
      ],
      [
        "3903335409076464950016842146214318381616743200272768251329673263329445521934",
        "8822745067031219301073330395613443753011843342528653757842007857532252394897",
        "1"
      ],
      [
        "20963169553880060624744369756206285183771006478629749548259884282180481836534",
        "20279716059842296973063211782744920064881111010681935031261606985597860986180",
        "1"
      ],
      [
        "5095299918745785358235521759906589570981860067522840247916967430706086094557",
        "12133158724583717328166109840120375488573482221575289550453667718133976528711",
        "1"
      ],
      [
        "6252243763528887479655373972663605655248654166160168398252973914846559499451",
        "19695240308799025883820642095687086351053987273168071182002606925852289407370",
        "1"
      ],
      [
        "4750713164113859748632553187685803530787446088636347100756780854479883104015",
        "16524898010325066819175496974692004268392693095676683693888933219400161831678",
        "1"
      ],
      [
        "8475730413189097684199719897159359674549695479833187546363581091151973456000",
        "6575024985438886780945419412068983299568680022782503793060009594499344573170",
        "1"
      ],
      [
        "16931660422068331333426421256087717976475391781393516266616339605910942675799",
        "10798018304952842957642633299117637424880355563081929085194665026052280411608",
        "1"
      ],
      [
        "17108305426117213840473123358110156865232158305904967287011703547445123818610",
        "4942568915479502343383295887000935910860792051791900836046749297657581173695",
        "1"
      ],
      [
        "3053330074625595859032994119330264137217008186816143155877315943458516609723",
        "10928277854685557763504975592825605803473667433076434954239097747587556401639",
        "1"
      ],
      [
        "14260996190257301724307327857473614480455053118165499642543870884065213664802",
        "5781043356584800534083724257045871515554831431206130331874662035818208730916",
        "1"
      ],
      [
        "13317606427169277258741630922600625957145741985911619244480545112853676645090",
        "572783024506068253346259078489068696797043226601655240843070730803225057046",
        "1"
      ],
      [
        "18833598692940968004238571946510816466217783250926189461491507496528773063245",
        "9803047670579031865037220222352573557736479776637366113346841421108087024999",
        "1"
      ],
      [
        "16863963222995263202348067448801398078392829688720499342469521212653985602487",
        "5519825027928147361149571760747081306325606883566669488697271744024490337713",
        "1"
      ],
      [
        "3290182205014312303396202753032137899158438221048761947449055699102872760091",
        "15851623347937411436723386196039681426964692538377995126973930110879276421219",
        "1"
      ],
      [
        "4864368932784106363139200909803344009482582568508303840147146481653699687116",
        "18819105934616626883514776813955507376198415537298913546567378213336674500185",
        "1"
      ],
      [
        "11621424319863845876772693521251285267934330301013583282886665290577107047754",
        "14757629890330056529060446934608077240991821050049361571332295098162959608138",
        "1"
      ],
      [
        "13313378820527301676075398954384468626068276316581551700749216148064958870827",
        "4570686841791817360077052993124537204919387072413391380697780106447560304021",
        "1"
      ],
      [
        "15129484242473909982576722937346346137003248414667516042498463392561426333395",
        "11521943800904290626919156263047047983059520411337770928798239813991641618194",
        "1"
      ],
      [
        "20961904026923155720762616161576867868311128903756310726865635905664533823460",
        "622058629612433662647233277839533079818226833910358972024383668685579913266",
        "1"
      ],
      [
        "1533622366571790629947432921314140452761040756481885352413149272711767937295",
        "17114490941702232460276930767791689779218278654921328370533051193019480334013",
        "1"
      ],
      [
        "3872345698339374374262636133638238401218696068128991467025014085621554417288",
        "5022659195638806340461724291337269719146700577992879438615182782548662972273",
        "1"
      ],
      [
        "1892716585138150130125401193013638707197028162928516613379540202811516362219",
        "5488777164986426669680999859949800986314280191938087219824281589742727483623",
        "1"
      ],
      [
        "14723470064749399568227840255239556884827479726776670054549910646833921784397",
        "9850312122776445437928702910599449626324211657013931974572474877352706350126",
        "1"
      ],
      [
        "9950769240264532726070836771187757315842167364102348677135477949114681025079",
        "3408332309304167992372902937582832393896979057510540217526111203670003315844",
        "1"
      ],
      [
        "18176981554172428201599098550569359991916865570367066111740022950649267830425",
        "1504425020122023771905878765060423416139787562985474457883000913071726901061",
        "1"
      ],
      [
        "8025346228433260850278166492321799112421131416178993792301266218315749244869",
        "18003902846005525205604781629324908788035926576838595439192554527651062180881",
        "1"
      ],
      [
        "7657371219832862030178333533911732352748584695586346243395458915778658564707",
        "15526170180937420482258756256789494136225570243659664056782104842571677681737",
        "1"
      ],
      [
        "1422092002462125993634365017842944329542993521984208494589273271206097414227",
        "19995906694386773441868898434490466214575501668430787350691378128976533171434",
        "1"
      ],
      [
        "13549896273100282761212147161568408582278060205778138880736758099020613619123",
        "14866480605200774667680263996986607619880834833436959561215039626981432470438",
        "1"
      ],
      [
        "3994741454453578842507365286256147027842650417717422946550104243267962330526",
        "20285002847260145366360747410192787771887613258937008542290569287903227281096",
        "1"
      ],
      [
        "13013905821442600236291763236258646172544367104636592517338414430282798068451",
        "88446066118379034768219930198938867468445931864040254837930141662737233637",
        "1"
      ],
      [
        "13626065954646412358003398584868006948208810234372685348960415372826734657148",
        "7801769257271052343527026775182064085614220780378411058204096830467909423973",
        "1"
      ],
      [
        "8180814095472527088987445511544901472649781074524520750015753401075124546192",
        "13265664615667652438789999049351647612638589194418028696721110024545209240245",
        "1"
      ],
      [
        "2117378660950893546727814473886971659167432730539153849939512739200702503245",
        "12205359061008714457233733605183853461049415901081652638746589684703684898085",
        "1"
      ],
      [
        "10566821862315077178806104928944101674606953837733742141460939837562283557106",
        "7674596940238494371146955016472389653206629774315881560174607338434178695752",
        "1"
      ],
      [
        "4113246305952002410836630545320048893831804716717760502322329658392516177157",
        "2582495162214693865485449556707643861588432316376893482956650698880036027760",
        "1"
      ],
      [
        "20256693581650511627508605728963644225029552513871906025264016420668844849954",
        "12046581620637748907292115969446085562219751931856730986934696177199258594367",
        "1"
      ],
      [
        "16623453222175488768803877928115854216457893375289846608149341380433335390264",
        "20980548229192775162392942720554544442201001541140648561609184738736036115588",
        "1"
      ],
      [
        "20568305659608905845211487930387411431885234200374506228290132380352892884377",
        "11307115280060459353713777285473062810900052439880886474317440718763946063629",
        "1"
      ],
      [
        "3838648357713189065800348378040071684774469060609379455364519829505660691767",
        "7059951936368799551213701438908864900907168553371150826060510959256874687494",
        "1"
      ],
      [
        "3507168370743824993280363915556711971116855778400710638483150695489655644035",
        "10450347161561586251232563671031812114693582330047082305628819792992289378888",
        "1"
      ],
      [
        "6289090165086218935848950899207186578398634671336111164113557140672876572076",
        "14303414163305840776877475218248221862336328016123213730298518782072837008926",
        "1"
      ]
    ],
    "nPublic": 65,
    "vk_alpha_1": [
      "20491192805390485299153009773594534940189261866228447918068658471970481763042",
      "9383485363053290200918347156157836566562967994039712273449902621266178545958",
      "1"
    ],
    "vk_beta_2": [
      [
        "6375614351688725206403948262868962793625744043794305715222011528459656738731",
        "4252822878758300859123897981450591353533073413197771768651442665752259397132"
      ],
      [
        "10505242626370262277552901082094356697409835680220590971873171140371331206856",
        "21847035105528745403288232691147584728191162732299865338377159692350059136679"
      ],
      [
        "1",
        "0"
      ]
    ],
    "vk_gamma_2": [
      [
        "10857046999023057135944570762232829481370756359578518086990519993285655852781",
        "11559732032986387107991004021392285783925812861821192530917403151452391805634"
      ],
      [
        "8495653923123431417604973247489272438418190587263600148770280649306958101930",
        "4082367875863433681332203403145435568316851327593401208105741076214120093531"
      ],
      [
        "1",
        "0"
      ]
    ],
    "vk_delta_2": [
      [
        "1284619067782625262033299823943842199378997899590458398651143911293904840584",
        "15230376273229886795002338330683690771549045328259379373615801464957198430450"
      ],
      [
        "7967187382060100406390096977719265945347583969849404952170747602356838614509",
        "21657085724858073143598952611779032379828005115043041502513345600210438172407"
      ],
      [
        "1",
        "0"
      ]
    ],
    "vk_alphabeta_12": [
      [
        [
          "2029413683389138792403550203267699914886160938906632433982220835551125967885",
          "21072700047562757817161031222997517981543347628379360635925549008442030252106"
        ],
        [
          "5940354580057074848093997050200682056184807770593307860589430076672439820312",
          "12156638873931618554171829126792193045421052652279363021382169897324752428276"
        ],
        [
          "7898200236362823042373859371574133993780991612861777490112507062703164551277",
          "7074218545237549455313236346927434013100842096812539264420499035217050630853"
        ]
      ],
      [
        [
          "7077479683546002997211712695946002074877511277312570035766170199895071832130",
          "10093483419865920389913245021038182291233451549023025229112148274109565435465"
        ],
        [
          "4595479056700221319381530156280926371456704509942304414423590385166031118820",
          "19831328484489333784475432780421641293929726139240675179672856274388269393268"
        ],
        [
          "11934129596455521040620786944827826205713621633706285934057045369193958244500",
          "8037395052364110730298837004334506829870972346962140206007064471173334027475"
        ]
      ]
    ],
    "curve": "bn128",
    "protocol": "groth16"
  }
}"#;

		let v = Verifier::from_json_u8_slice(vk.as_bytes()).unwrap();

		assert_eq!("bn128", v.vk_json.curve);
		assert_eq!("groth16", v.vk_json.protocol);
		assert_eq!(65, v.vk_json.inputs_count);
		assert_eq!(
			"20491192805390485299153009773594534940189261866228447918068658471970481763042",
			v.vk_json.vk_alpha_1[0]
		);
		assert_eq!(
			"4252822878758300859123897981450591353533073413197771768651442665752259397132",
			v.vk_json.vk_beta_2[0][1]
		);
		assert_eq!(
			"4082367875863433681332203403145435568316851327593401208105741076214120093531",
			v.vk_json.vk_gamma_2[1][1]
		);
		assert_eq!(
			"2029413683389138792403550203267699914886160938906632433982220835551125967885",
			v.vk_json.vk_alphabeta_12[0][0][0]
		);
		assert_eq!(
			"13896811173741308528708014989807693837248126243365939233014522115172481095858",
			v.vk_json.ic[2][1]
		);

		let res = zk_light_client_rotate(&lcs, v);

		assert_ok!(res.clone());
		assert_eq!(true, res.unwrap());
	}
}
