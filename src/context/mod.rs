use tera::{Context, Tera};

mod funcs;
mod data;

pub fn get_context() -> Context {

    // Create a new Tera context, make it mutable
    let mut context = Context::new();

    // NOTE: Pass the context through each of these different things to fill
    let _ = data::get_wakatime_stats(&mut context);

    return context;
}

pub fn register_funcs(tera : &mut Tera) -> Result<(), String> {

    tera.register_function(
        "progressbar", 
        funcs::progressbar::Progressbar::builder()
            .char_length(16)
    );

    Ok(())
}