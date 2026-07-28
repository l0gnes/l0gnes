use std::{any::Any, collections::HashMap, hash::Hash};

pub enum ContextBuildError {

}

/// This function is repsonsible for producing the main context that will be passed to the
/// render call.
pub fn build_context(cmdline_args : &crate::args::CmdlineArgs) -> Result<tera::Context, ContextBuildError> {

    // Create a new context instance
    let ctx = tera::Context::new();

    return Ok(ctx);

}

// Builds a mapping which shows us where all the keys might come from
fn build_provider_mapping<T: ReadmeToolContextPlugin>(plugins: Vec<&T>) -> HashMap<&'static str, Vec<&T>> {
    
    let mut map = HashMap::<&'static str, Vec<&T>>::new();

    plugins
        .iter()
        .for_each(|p| {
            p.provides_keys()
                .iter()
                .for_each(|k| {
                    let k_entry = map.entry(*k)
                        .or_insert(vec![])
                        .push(*p);
                });
        });
    return map;
}

fn build_dependency_mapping<T: ReadmeToolContextPlugin>(
    plugins: Vec<&T>, 
    providers: HashMap<&'static str, Vec<&T>>
) 
{

    

}

fn validate_ctx_plugin_keys<T: ReadmeToolContextPlugin>(plugin: &T) -> bool {

    // Probably needs expanding, but this should prevent any simple stupid mistakes
    return !plugin.requires_keys()
        .iter().any(|key| plugin.provides_keys().contains(key))

}

pub trait ReadmeToolContextPlugin {
    fn extend_ctx(&self, ctx: &mut tera::Context) -> Result<(), ContextBuildError>;

    fn requires_keys(&self) -> Vec<&'static str>;

    fn provides_keys(&self) -> Vec<&'static str>;
}