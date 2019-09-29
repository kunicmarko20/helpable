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
                    return Self::choose(&github_client, repository_name)
                        .and_then(|pull_request| Ok(pull_request.pull_request_number()));
                }

                Ok(pull_request_number.unwrap())
            }

            fn choose(github_client: &github_client::github::GithubClient, repository_name: &str) -> Result<github_client::github::payload::PullRequestPayload, String> {
                let pull_requests = github_client.list_pull_requests(repository_name);

                let selections: Vec<&str> = pull_requests
                    .iter()
                    .map(|pull_request| pull_request.title())
                    .collect();

                if selections.is_empty() {
                    return Err("No Pull Requests found in repository.".to_string());
                }

                let selected = dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
                    .with_prompt("Choose Pull Request:")
                    .items(&selections[..])
                    .interact()
                    .unwrap();

                let selected_title = selections[selected].to_string();

                Ok(pull_requests
                    .into_iter()
                    .find(|pull_request| pull_request.title() == selected_title)
                    .expect("Unable to find matching Pull Request.")
                )
            }
        }
    };

    gen.into()
}
