#[macro_use]
extern crate quote;
extern crate proc_macro;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(ChoosablePullRequest)]
pub fn choosable_pull_request(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_choosable_pull_request(&ast)
}

fn impl_choosable_pull_request(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl #name {
            fn pull_request_number(
                pull_request_number: Option<u64>,
                github_client: &github_client::github::GithubClient,
                repository_name: &str,
            ) -> Result<u64, String> {
                if pull_request_number.is_none() {
                    let pull_requests = github_client.list_pull_requests(repository_name);

                    let chosen = Self::choose(
                        pull_requests
                            .iter()
                            .map(|pull_request| pull_request.title())
                            .collect(),
                        repository_name
                    )?;

                    return Ok(pull_requests
                        .into_iter()
                        .find(|pull_request| pull_request.title() == chosen)
                        .expect("Unable to find matching Pull Request.")
                        .pull_request_number()
                    );
                }

                Ok(pull_request_number.unwrap())
            }

            fn choose(selections: Vec<&str>, repository_name: &str) -> Result<String, String> {
                if selections.is_empty() {
                    return Err("No Pull Requests found in repository.".to_string());
                }

                let selected = dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
                    .with_prompt("Choose Pull Request:")
                    .items(&selections[..])
                    .interact()
                    .unwrap();

                Ok(selections[selected].to_string())
            }
        }
    };

    gen.into()
}
