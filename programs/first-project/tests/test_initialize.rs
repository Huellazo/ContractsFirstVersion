use {
    anchor_lang::{
        prelude::Pubkey,
        solana_program::{instruction::Instruction, system_program},
        AccountDeserialize, InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
};

fn program_id() -> Pubkey {
    first_project::id()
}

fn send(
    svm: &mut LiteSVM,
    payer: &Keypair,
    ix: Instruction,
    extra_signers: &[&Keypair],
) -> Result<(), Box<dyn std::error::Error>> {
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &blockhash);
    let mut signers: Vec<&dyn solana_signer::Signer> = vec![payer];
    for s in extra_signers {
        signers.push(*s);
    }
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &signers).unwrap();
    svm.send_transaction(tx)
        .map_err(|e| -> Box<dyn std::error::Error> { format!("{e:?}").into() })?;
    Ok(())
}

#[test]
fn test_huellazo_flow() {
    let pid = program_id();
    let payer = Keypair::new();
    let admin = Keypair::new();
    let business_wallet = Keypair::new();

    let mut svm = LiteSVM::new();
    let bytes = include_bytes!(concat!(
        env!("CARGO_TARGET_TMPDIR"),
        "/../deploy/first_project.so"
    ));
    svm.add_program(pid, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();
    svm.airdrop(&admin.pubkey(), 10_000_000_000).unwrap();
    svm.airdrop(&business_wallet.pubkey(), 10_000_000_000).unwrap();

    // 1. init config
    let config = Pubkey::find_program_address(&[first_project::constants::CONFIG_SEED], &pid).0;
    let ix = Instruction::new_with_bytes(
        pid,
        &first_project::instruction::InitConfig {
            backend_admin: Pubkey::new_unique(),
        }
        .data(),
        first_project::accounts::InitConfig {
            admin: admin.pubkey(),
            config,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );
    send(&mut svm, &admin, ix, &[]).unwrap();

    // 2. initialize passport
    let passport = Pubkey::find_program_address(
        &[first_project::constants::PASSPORT_SEED, payer.pubkey().as_ref()],
        &pid,
    )
    .0;
    let ix = Instruction::new_with_bytes(
        pid,
        &first_project::instruction::InitializePassport {}.data(),
        first_project::accounts::InitializePassport {
            user: payer.pubkey(),
            passport,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );
    send(&mut svm, &payer, ix, &[]).unwrap();
    let acc = svm.get_account(&passport).unwrap();
    let mut data: &[u8] = &acc.data;
    let p = first_project::state::Passport::try_deserialize(&mut data).unwrap();
    assert_eq!(p.owner, payer.pubkey());
    assert_eq!(p.total_xp, 0);

    // 3. create tourist spot
    let spot_id: u64 = 1u64;
    let spot = Pubkey::find_program_address(
        &[first_project::constants::TOURIST_SPOT_SEED, &spot_id.to_le_bytes()],
        &pid,
    )
    .0;
    let ix = Instruction::new_with_bytes(
        pid,
        &first_project::instruction::CreateTouristSpot {
            id: spot_id,
            name: "Torre Eiffel".to_string(),
            latitude: 488_588_880,
            longitude: 23_295_000,
            radius: 50,
            base_xp_reward: 1000,
            achievement_uri: "ipfs://torre-eiffel".to_string(),
        }
        .data(),
        first_project::accounts::CreateTouristSpot {
            admin: admin.pubkey(),
            config,
            spot,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );
    send(&mut svm, &admin, ix, &[]).unwrap();
    let acc = svm.get_account(&spot).unwrap();
    let mut d: &[u8] = &acc.data;
    let s = first_project::state::TouristSpot::try_deserialize(&mut d).unwrap();
    assert_eq!(s.name, "Torre Eiffel");
    assert!(s.is_active);

    // 4. create business
    let business = Pubkey::find_program_address(
        &[first_project::constants::BUSINESS_SEED, business_wallet.pubkey().as_ref()],
        &pid,
    )
    .0;
    let ix = Instruction::new_with_bytes(
        pid,
        &first_project::instruction::CreateBusiness {
            name: "Café Huellazo".to_string(),
        }
        .data(),
        first_project::accounts::CreateBusiness {
            authority: business_wallet.pubkey(),
            business,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );
    send(&mut svm, &business_wallet, ix, &[]).unwrap();

    // 5. create business mission
    let mission_id: u64 = 7u64;
    let mission = Pubkey::find_program_address(
        &[first_project::constants::BUSINESS_MISSION_SEED, &mission_id.to_le_bytes()],
        &pid,
    )
    .0;
    let ix = Instruction::new_with_bytes(
        pid,
        &first_project::instruction::CreateBusinessMission {
            id: mission_id,
            name: "Ruta del Café".to_string(),
            required_check_ins: 1,
            target_businesses: vec![business_wallet.pubkey()],
            reward_discount: 20,
            expires_at: 0,
        }
        .data(),
        first_project::accounts::CreateBusinessMission {
            business_group: business_wallet.pubkey(),
            mission,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );
    send(&mut svm, &business_wallet, ix, &[]).unwrap();

    // 6. create discount
    let discount_id: u64 = 3u64;
    let discount = Pubkey::find_program_address(
        &[
            first_project::constants::DISCOUNT_SEED,
            business_wallet.pubkey().as_ref(),
            &discount_id.to_le_bytes(),
        ],
        &pid,
    )
    .0;
    let ix = Instruction::new_with_bytes(
        pid,
        &first_project::instruction::CreateDiscount {
            id: discount_id,
            code: "CAFÉ15".to_string(),
            discount_percent: 15,
            min_purchase: 1_000_000,
            available_units: 5,
            valid_until: 0,
        }
        .data(),
        first_project::accounts::CreateDiscount {
            business: business_wallet.pubkey(),
            business_account: business,
            discount,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );
    send(&mut svm, &business_wallet, ix, &[]).unwrap();

    // 7. check in business (with discount + mission)
    let timestamp: i64 = 1_700_000_000;
    let check_in = Pubkey::find_program_address(
        &[
            first_project::constants::BUSINESS_CHECKIN_SEED,
            payer.pubkey().as_ref(),
            business_wallet.pubkey().as_ref(),
            &timestamp.to_le_bytes(),
        ],
        &pid,
    )
    .0;
    let progress = Pubkey::find_program_address(
        &[
            first_project::constants::BUSINESS_PROGRESS_SEED,
            payer.pubkey().as_ref(),
            &mission_id.to_le_bytes(),
        ],
        &pid,
    )
    .0;
    let ix = Instruction::new_with_bytes(
        pid,
        &first_project::instruction::CheckInBusiness {
            amount_spent: 2_000_000,
            rating: 5,
            use_discount: true,
            discount_id: Some(discount_id),
            mission_id: Some(mission_id),
            timestamp,
        }
        .data(),
        first_project::accounts::CheckInBusiness {
            user: payer.pubkey(),
            business_wallet: business_wallet.pubkey(),
            business_account: business,
            passport,
            check_in,
            discount: Some(discount),
            mission: Some(mission),
            mission_progress: Some(progress),
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );
    send(&mut svm, &payer, ix, &[]).unwrap();

    let acc = svm.get_account(&passport).unwrap();
    let mut d: &[u8] = &acc.data;
    let p = first_project::state::Passport::try_deserialize(&mut d).unwrap();
    assert!(p.total_xp > 0, "xp should have been awarded");

    let acc = svm.get_account(&check_in).unwrap();
    let mut d: &[u8] = &acc.data;
    let c = first_project::state::BusinessCheckIn::try_deserialize(&mut d).unwrap();
    assert_eq!(c.rating, 5);
    assert!(c.used_discount);
    assert_eq!(c.discount_applied, 15);

    let acc = svm.get_account(&progress).unwrap();
    let mut d: &[u8] = &acc.data;
    let prog = first_project::state::UserBusinessMissionProgress::try_deserialize(&mut d).unwrap();
    assert!(prog.completed);
}