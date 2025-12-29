#[macro_export]
macro_rules! mybox {
    ($val: expr) => {
        Box::new($val)
    }
}
#[macro_export]
macro_rules! make_error {
    ($sentence:expr, $label:expr, $span:expr) => {
        LabeledError::new($sentence.to_string())
        .with_label($label, $span)
    };
}

#[macro_export]
macro_rules! return_error {
    ($sentence:expr, $label:expr, $span:expr) => {
        return Err(make_error!($sentence, $label, $span))
    };
}

#[macro_export]
macro_rules! plugincmd {
    (
        plugin: $pluginName: ty,
        name: $commandStructName: ident,
        cliName: $commandCliName: expr,
        signature: $signature: expr,
        description: $description: expr,
        searchTerms: [ $($search_terms: expr),* $(,)?],
        $(examples: [ $( $examples: expr),* $(,)?])? $(,)?
        run: $run: expr
        $(,)? //Allow Trailing commas
    ) => {
        #[allow(unused_imports)] //Suppress Warnings.
        use nu_protocol::{
            Signature,
            Example,
            PipelineData,
            SyntaxShape,
            LabeledError,
            Type,
            Value
        };
        use nu_plugin::{
            EngineInterface,
            EvaluatedCall,
        };
        pub struct $commandStructName;
        impl nu_plugin::PluginCommand for $commandStructName {
            type Plugin = $pluginName;
            fn name(&self) -> &str {$commandCliName}
            fn signature(&self) -> Signature {let sig_eval = $signature; sig_eval}
            fn description(&self) -> &str {
                $description
            }
            fn examples(&self) -> Vec<Example<'_>> {
                ::std::vec![
                    $($($examples),*)*
                ]
            }
            fn search_terms(&self) -> Vec<&str> {
                ::std::vec![
                    $($search_terms),*
                ]
            }
            fn run(&self,
            plugin: &Self::Plugin,
            engine: &EngineInterface,
            call: &EvaluatedCall,
            input: PipelineData)
            -> Result<PipelineData, LabeledError> {
                $run(plugin, engine, call, input)
            }
        }
    }
}

#[macro_export]
macro_rules! debugf {
    ($str: expr $(,$arguments:expr)* $(,)?) => {{
        if cfg!(debug_assertions) {
            // let str:&str = $str; //Enforce a &str type.
            eprintln!(
                $str,
                $( $arguments )*
            ) //print to stderr instead of stdout to ensure the output doesnt get caught by any piping.
        }
    }}
}