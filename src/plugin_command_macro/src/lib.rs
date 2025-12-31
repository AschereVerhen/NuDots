use darling::FromMeta;
use proc_macro::TokenStream;
use syn::{
    ItemStruct, parse_macro_input
};
use quote::quote;


fn vec_default_val() -> Vec<syn::LitStr> {
    vec![]
}

fn option_default_val() -> Option<syn::Expr> {
    None
}

#[derive(darling::FromMeta)]
struct PluginCommandArgs {
    name: String,
    plugin: syn::Path,
    #[darling(rename = "signature")]
    sig: syn::Expr,
    //We will let the user write their own run().
    description: syn::LitStr,
    #[darling(default = || vec_default_val())]
    search_terms: Vec<syn::LitStr>,
    //Let users write their own examples as it would get cluttered here... but we can just take the expr of it aswell
    #[darling(default = || option_default_val())]
    examples: Option<syn::Expr>,
    // The below is not used as a struct it requires is deprecated.
    // #[darling(default = || option_default_val())]
    // dynamic_completion: Option<syn::Expr>,
    run: syn::Path,
}


#[proc_macro_attribute]
pub fn plugin_command(attr: TokenStream, input: TokenStream) -> TokenStream {
    //First lets only support plugin and name.

    let attr_args = match darling::ast::NestedMeta::parse_meta_list(attr.into()) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(darling::Error::from(e).write_errors()),
    };

    let args = match PluginCommandArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors())
    };
    let input = parse_macro_input!(input as ItemStruct);

    let name = &args.name;
    let plugin = &args.plugin;
    let signature = &args.sig;
    let desc = &args.description;
    let search_terms = if !args.search_terms.is_empty() {
        let terms = &args.search_terms;
        quote! {
            .search_terms(vec![ #(#terms.to_string()),* ])
        }
    } else {
        quote!(vec![])
    };
    let examples = if let Some(eg) = &args.examples {
        quote! {#eg}
    } else {
        quote! {vec![]}
    };
    let run = &args.run;
    let struct_name = &input.ident;
    let expanded = quote::quote! {
        #input

        impl ::nu_plugin::PluginCommand for #struct_name {
            type Plugin = #plugin;
            fn name(&self) -> &str {
                #name
            }
            fn signature(&self) -> nu_protocol::Signature {let sig_eval = #signature; sig_eval}
            fn examples(&self) -> Vec<::nu_protocol::Example<'_>> {
                #examples
            }
            fn description(&self) -> &str {
                #desc
            }

            fn search_terms(&self) -> Vec<&str> {
                #search_terms
            }

            fn run(
                &self,
                _plugin: &Self::Plugin,
                engine: &nu_plugin::EngineInterface,
                call: &nu_plugin::EvaluatedCall,
                input: nu_protocol::PipelineData
            )   -> Result<nu_protocol::PipelineData, nu_protocol::LabeledError> {
                #run(engine.clone(), call.clone(), input)
            }

        }
    };
    return TokenStream::from(expanded)
}