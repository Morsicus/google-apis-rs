// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *Shopping Content* crate version *1.0.0+20160905*, where *20160905* is the exact revision of the *content:v2* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.0*.
//! 
//! Everything else about the *Shopping Content* *v2* API can be found at the
//! [official documentation site](https://developers.google.com/shopping-content).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/content2).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.ShoppingContent.html) ... 
//! 
//! * [accounts](struct.Account.html)
//!  * [*authinfo*](struct.AccountAuthinfoCall.html), [*custombatch*](struct.AccountCustombatchCall.html), [*delete*](struct.AccountDeleteCall.html), [*get*](struct.AccountGetCall.html), [*insert*](struct.AccountInsertCall.html), [*list*](struct.AccountListCall.html), [*patch*](struct.AccountPatchCall.html) and [*update*](struct.AccountUpdateCall.html)
//! * accountshipping
//!  * [*custombatch*](struct.AccountshippingCustombatchCall.html), [*get*](struct.AccountshippingGetCall.html), [*list*](struct.AccountshippingListCall.html), [*patch*](struct.AccountshippingPatchCall.html) and [*update*](struct.AccountshippingUpdateCall.html)
//! * accountstatuses
//!  * [*custombatch*](struct.AccountstatuseCustombatchCall.html), [*get*](struct.AccountstatuseGetCall.html) and [*list*](struct.AccountstatuseListCall.html)
//! * accounttax
//!  * [*custombatch*](struct.AccounttaxCustombatchCall.html), [*get*](struct.AccounttaxGetCall.html), [*list*](struct.AccounttaxListCall.html), [*patch*](struct.AccounttaxPatchCall.html) and [*update*](struct.AccounttaxUpdateCall.html)
//! * [datafeeds](struct.Datafeed.html)
//!  * [*custombatch*](struct.DatafeedCustombatchCall.html), [*delete*](struct.DatafeedDeleteCall.html), [*get*](struct.DatafeedGetCall.html), [*insert*](struct.DatafeedInsertCall.html), [*list*](struct.DatafeedListCall.html), [*patch*](struct.DatafeedPatchCall.html) and [*update*](struct.DatafeedUpdateCall.html)
//! * datafeedstatuses
//!  * [*custombatch*](struct.DatafeedstatuseCustombatchCall.html), [*get*](struct.DatafeedstatuseGetCall.html) and [*list*](struct.DatafeedstatuseListCall.html)
//! * [inventory](struct.Inventory.html)
//!  * [*custombatch*](struct.InventoryCustombatchCall.html) and [*set*](struct.InventorySetCall.html)
//! * [orders](struct.Order.html)
//!  * [*acknowledge*](struct.OrderAcknowledgeCall.html), [*advancetestorder*](struct.OrderAdvancetestorderCall.html), [*cancel*](struct.OrderCancelCall.html), [*cancellineitem*](struct.OrderCancellineitemCall.html), [*createtestorder*](struct.OrderCreatetestorderCall.html), [*custombatch*](struct.OrderCustombatchCall.html), [*get*](struct.OrderGetCall.html), [*getbymerchantorderid*](struct.OrderGetbymerchantorderidCall.html), [*gettestordertemplate*](struct.OrderGettestordertemplateCall.html), [*list*](struct.OrderListCall.html), [*refund*](struct.OrderRefundCall.html), [*returnlineitem*](struct.OrderReturnlineitemCall.html), [*shiplineitems*](struct.OrderShiplineitemCall.html), [*updatemerchantorderid*](struct.OrderUpdatemerchantorderidCall.html) and [*updateshipment*](struct.OrderUpdateshipmentCall.html)
//! * [products](struct.Product.html)
//!  * [*custombatch*](struct.ProductCustombatchCall.html), [*delete*](struct.ProductDeleteCall.html), [*get*](struct.ProductGetCall.html), [*insert*](struct.ProductInsertCall.html) and [*list*](struct.ProductListCall.html)
//! * productstatuses
//!  * [*custombatch*](struct.ProductstatuseCustombatchCall.html), [*get*](struct.ProductstatuseGetCall.html) and [*list*](struct.ProductstatuseListCall.html)
//! * shippingsettings
//!  * [*custombatch*](struct.ShippingsettingCustombatchCall.html), [*get*](struct.ShippingsettingGetCall.html), [*getsupportedcarriers*](struct.ShippingsettingGetsupportedcarrierCall.html), [*list*](struct.ShippingsettingListCall.html), [*patch*](struct.ShippingsettingPatchCall.html) and [*update*](struct.ShippingsettingUpdateCall.html)
//! 
//! 
//! 
//! 
//! Not what you are looking for ? Find all other Google APIs in their Rust [documentation index](../index.html).
//! 
//! # Structure of this Library
//! 
//! The API is structured into the following primary items:
//! 
//! * **[Hub](struct.ShoppingContent.html)**
//!     * a central object to maintain state and allow accessing all *Activities*
//!     * creates [*Method Builders*](trait.MethodsBuilder.html) which in turn
//!       allow access to individual [*Call Builders*](trait.CallBuilder.html)
//! * **[Resources](trait.Resource.html)**
//!     * primary types that you can apply *Activities* to
//!     * a collection of properties and *Parts*
//!     * **[Parts](trait.Part.html)**
//!         * a collection of properties
//!         * never directly used in *Activities*
//! * **[Activities](trait.CallBuilder.html)**
//!     * operations to apply to *Resources*
//! 
//! All *structures* are marked with applicable traits to further categorize them and ease browsing.
//! 
//! Generally speaking, you can invoke *Activities* like this:
//! 
//! ```Rust,ignore
//! let r = hub.resource().activity(...).doit()
//! ```
//! 
//! Or specifically ...
//! 
//! ```ignore
//! let r = hub.orders().get(...).doit()
//! let r = hub.orders().list(...).doit()
//! let r = hub.orders().updateshipment(...).doit()
//! let r = hub.orders().advancetestorder(...).doit()
//! let r = hub.orders().updatemerchantorderid(...).doit()
//! let r = hub.orders().returnlineitem(...).doit()
//! let r = hub.orders().gettestordertemplate(...).doit()
//! let r = hub.orders().createtestorder(...).doit()
//! let r = hub.orders().refund(...).doit()
//! let r = hub.orders().custombatch(...).doit()
//! let r = hub.orders().cancellineitem(...).doit()
//! let r = hub.orders().getbymerchantorderid(...).doit()
//! let r = hub.orders().acknowledge(...).doit()
//! let r = hub.orders().cancel(...).doit()
//! let r = hub.orders().shiplineitems(...).doit()
//! ```
//! 
//! The `resource()` and `activity(...)` calls create [builders][builder-pattern]. The second one dealing with `Activities` 
//! supports various methods to configure the impending operation (not shown here). It is made such that all required arguments have to be 
//! specified right away (i.e. `(...)`), whereas all optional ones can be [build up][builder-pattern] as desired.
//! The `doit()` method performs the actual communication with the server and returns the respective result.
//! 
//! # Usage
//! 
//! ## Setting up your Project
//! 
//! To use this library, you would put the following lines into your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! google-content2 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_content2 as content2;
//! use content2::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use content2::ShoppingContent;
//! 
//! // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
//! // `client_secret`, among other things.
//! let secret: ApplicationSecret = Default::default();
//! // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
//! // unless you replace  `None` with the desired Flow.
//! // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
//! // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
//! // retrieve them from storage.
//! let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
//!                               hyper::Client::new(),
//!                               <MemoryStorage as Default>::default(), None);
//! let mut hub = ShoppingContent::new(hyper::Client::new(), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.orders().list("merchantId")
//!              .add_statuses("sit")
//!              .placed_date_start("takimata")
//!              .placed_date_end("elitr")
//!              .page_token("nonumy")
//!              .order_by("rebum.")
//!              .max_results(95)
//!              .acknowledged(true)
//!              .doit();
//! 
//! match result {
//!     Err(e) => match e {
//!         // The Error enum provides details about what exactly happened.
//!         // You can also just use its `Debug`, `Display` or `Error` traits
//!          Error::HttpError(_)
//!         |Error::MissingAPIKey
//!         |Error::MissingToken(_)
//!         |Error::Cancelled
//!         |Error::UploadSizeLimitExceeded(_, _)
//!         |Error::Failure(_)
//!         |Error::BadRequest(_)
//!         |Error::FieldClash(_)
//!         |Error::JsonDecodeError(_, _) => println!("{}", e),
//!     },
//!     Ok(res) => println!("Success: {:?}", res),
//! }
//! # }
//! ```
//! ## Handling Errors
//! 
//! All errors produced by the system are provided either as [Result](enum.Result.html) enumeration as return value of 
//! the doit() methods, or handed as possibly intermediate results to either the 
//! [Hub Delegate](trait.Delegate.html), or the [Authenticator Delegate](../yup-oauth2/trait.AuthenticatorDelegate.html).
//! 
//! When delegates handle errors or intermediate values, they may have a chance to instruct the system to retry. This 
//! makes the system potentially resilient to all kinds of errors.
//! 
//! ## Uploads and Downloads
//! If a method supports downloads, the response body, which is part of the [Result](enum.Result.html), should be
//! read by you to obtain the media.
//! If such a method also supports a [Response Result](trait.ResponseResult.html), it will return that by default.
//! You can see it as meta-data for the actual media. To trigger a media download, you will have to set up the builder by making
//! this call: `.param("alt", "media")`.
//! 
//! Methods supporting uploads can do so using up to 2 different protocols: 
//! *simple* and *resumable*. The distinctiveness of each is represented by customized 
//! `doit(...)` methods, which are then named `upload(...)` and `upload_resumable(...)` respectively.
//! 
//! ## Customization and Callbacks
//! 
//! You may alter the way an `doit()` method is called by providing a [delegate](trait.Delegate.html) to the 
//! [Method Builder](trait.CallBuilder.html) before making the final `doit()` call. 
//! Respective methods will be called to provide progress information, as well as determine whether the system should 
//! retry on failure.
//! 
//! The [delegate trait](trait.Delegate.html) is default-implemented, allowing you to customize it with minimal effort.
//! 
//! ## Optional Parts in Server-Requests
//! 
//! All structures provided by this library are made to be [enocodable](trait.RequestValue.html) and 
//! [decodable](trait.ResponseResult.html) via *json*. Optionals are used to indicate that partial requests are responses 
//! are valid.
//! Most optionals are are considered [Parts](trait.Part.html) which are identifiable by name, which will be sent to 
//! the server to indicate either the set parts of the request or the desired parts in the response.
//! 
//! ## Builder Arguments
//! 
//! Using [method builders](trait.CallBuilder.html), you are able to prepare an action call by repeatedly calling it's methods.
//! These will always take a single argument, for which the following statements are true.
//! 
//! * [PODs][wiki-pod] are handed by copy
//! * strings are passed as `&str`
//! * [request values](trait.RequestValue.html) are moved
//! 
//! Arguments will always be copied or cloned into the builder, to make them independent of their original life times.
//! 
//! [wiki-pod]: http://en.wikipedia.org/wiki/Plain_old_data_structure
//! [builder-pattern]: http://en.wikipedia.org/wiki/Builder_pattern
//! [google-go-api]: https://github.com/google/google-api-go-client
//! 
//! 

// Unused attributes happen thanks to defined, but unused structures
// We don't warn about this, as depending on the API, some data structures or facilities are never used.
// Instead of pre-determining this, we just disable the lint. It's manually tuned to not have any 
// unused imports in fully featured APIs. Same with unused_mut ... .
#![cfg_attr(feature = "nightly", feature(proc_macro))]
#![allow(unused_imports, unused_mut, dead_code)]

#[cfg(feature = "nightly")]
include!("lib.rs.in");

#[cfg(feature = "with-serde-codegen")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));