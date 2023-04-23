use std::process;

pub fn interface_list() -> anyhow::Result<Vec<WgInterface>> {
    let out = process::Command()::new("wg").arg("show").arg("interfaces").output()?;
    let ifc_strings = out.stdout.split(" ").collect::<Vec<&str>>();
}