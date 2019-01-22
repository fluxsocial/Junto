#[derive(Serialize, Deserialize, Debug, DefaultJson)]
pub struct MyEntry {
    content: String,
}

pub fn handle_create_my_entry(entry: MyEntry) -> ZomeApiResult<Address> {
    let entry = Entry::App("my_entry".into(), entry.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn handle_get_my_entry(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address);
}

fn definition() -> ValidatingEntryType {
    entry!(
        name: "my_entry",
        description: "this is a same entry defintion",
        sharing: Sharing::Public,
        native_type: MyEntry,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: |_my_entry: MyEntry, _ctx: hdk::ValidationData| {
            Ok(())
        }
    )
}
define_zome! {
    entries: [
       definition()
    ]

    genesis: || { Ok(()) }

    functions: {
        main (Public) {
            create_my_entry: {
                inputs: |entry: MyEntry|,
                outputs: |result: ZomeApiResult<Address>|,
                handler: handle_create_my_entry
            }
            get_my_entry: {
                inputs: |address: Address|,
                outputs: |result: ZomeApiResult<Option<Entry>>|,
                handler: handle_get_my_entry
            }
        }
    }
}
