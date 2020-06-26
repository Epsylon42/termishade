use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

fn variant_matcher(variant: &synstructure::VariantInfo) -> TokenStream {
    let mut variant = variant.clone();
    variant.binding_name(|_, i| Ident::new(&format!("__other_{}", i), Span::call_site()));

    for binding in variant.bindings_mut() {
        binding.style = synstructure::BindStyle::Move;
    }

    variant.pat()
}

fn derive_interpolate(mut s: synstructure::Structure) -> TokenStream {
    for variant in s.variants_mut() {
        for binding in variant.bindings_mut() {
            binding.style = synstructure::BindStyle::Move;
        }
    }

    let add_body = s.each_variant(|variant| {
        let pat = variant_matcher(&variant);
        let construct = variant.construct(|_, i| {
            let self_binding = Ident::new(&format!("__binding_{}", i), Span::call_site());
            let other_binding = Ident::new(&format!("__other_{}", i), Span::call_site());
            quote! {
                #self_binding + #other_binding
            }
        });

        quote! {
            match other {
                #pat => #construct,
                _ => panic!("Variants don't match")
            }
        }
    });

    let mul_body = s.each_variant(|variant| {
        variant.construct(|_, i| {
            let self_binding = Ident::new(&format!("__binding_{}", i), Span::call_site());
            quote! {
                #self_binding * coef
            }
        })
    });

    s.gen_impl(quote! {
        gen impl std::ops::Add<Self> for @Self {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                match self {
                    #add_body
                }
            }
        }

        gen impl std::ops::Mul<f32> for @Self {
            type Output = Self;

            fn mul(self, coef: f32) -> Self {
                match self {
                    #mul_body
                }
            }
        }
    })
}

synstructure::decl_derive!([Interpolate] => derive_interpolate);
