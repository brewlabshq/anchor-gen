pub use anchor_lang_idl_spec::*;
use heck::{ToPascalCase, ToSnakeCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// Generates a list of [IdlAccountItem]s as a [TokenStream].
pub fn generate_account_fields(
    name: &str,
    accounts: &[IdlInstructionAccountItem],
) -> (TokenStream, TokenStream) {
    let mut all_structs: Vec<TokenStream> = vec![];
    let all_fields = accounts
        .iter()
        .map(|account| match account {
            IdlInstructionAccountItem::Single(info) => {
                let acc_name = format_ident!("{}", info.name.to_snake_case());
                let annotation = if info.writable {
                    quote! { #[account(mut)] }
                } else {
                    quote! {}
                };
                let ty = if info.signer {
                    quote! { Signer<'info> }
                } else {
                    quote! { AccountInfo<'info> }
                };
                quote! {
                   #annotation
                   pub #acc_name: #ty
                }
            }
            IdlInstructionAccountItem::Composite(inner) => {
                let field_name = format_ident!("{}{}", name, inner.name.to_snake_case());
                let sub_name = format!("{}{}", name, inner.name.to_pascal_case());
                let sub_ident = format_ident!("{}", &sub_name);
                let (sub_structs, sub_fields) = generate_account_fields(&sub_name, &inner.accounts);
                all_structs.push(sub_structs);
                all_structs.push(quote! {
                    #[derive(Accounts)]
                    pub struct #sub_ident<'info> {
                        #sub_fields
                    }
                });
                quote! {
                    pub #field_name: #sub_ident<'info>
                }
            }
        })
        .collect::<Vec<_>>();
    (
        quote! {
            #(#all_structs)*
        },
        quote! {
            #(#all_fields),*
        },
    )
}
