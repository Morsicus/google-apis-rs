// DO NOT EDIT !
// This file was generated automatically from 'src/mako/api/lib.rs.mako'
// DO NOT EDIT !

//! This documentation was generated from *classroom* crate version *1.0.0+20160816*, where *20160816* is the exact revision of the *classroom:v1* schema built by the [mako](http://www.makotemplates.org/) code generator *v1.0.0*.
//! 
//! Everything else about the *classroom* *v1* API can be found at the
//! [official documentation site](https://developers.google.com/classroom/).
//! The original source code is [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/classroom1).
//! # Features
//! 
//! Handle the following *Resources* with ease from the central [hub](struct.Classroom.html) ... 
//! 
//! * [courses](struct.Course.html)
//!  * [*aliases create*](struct.CourseAliaseCreateCall.html), [*aliases delete*](struct.CourseAliaseDeleteCall.html), [*aliases list*](struct.CourseAliaseListCall.html), [*course work create*](struct.CourseCourseWorkCreateCall.html), [*course work get*](struct.CourseCourseWorkGetCall.html), [*course work list*](struct.CourseCourseWorkListCall.html), [*course work student submissions get*](struct.CourseCourseWorkStudentSubmissionGetCall.html), [*course work student submissions list*](struct.CourseCourseWorkStudentSubmissionListCall.html), [*course work student submissions modify attachments*](struct.CourseCourseWorkStudentSubmissionModifyAttachmentCall.html), [*course work student submissions patch*](struct.CourseCourseWorkStudentSubmissionPatchCall.html), [*course work student submissions reclaim*](struct.CourseCourseWorkStudentSubmissionReclaimCall.html), [*course work student submissions return*](struct.CourseCourseWorkStudentSubmissionReturnCall.html), [*course work student submissions turn in*](struct.CourseCourseWorkStudentSubmissionTurnInCall.html), [*create*](struct.CourseCreateCall.html), [*delete*](struct.CourseDeleteCall.html), [*get*](struct.CourseGetCall.html), [*list*](struct.CourseListCall.html), [*patch*](struct.CoursePatchCall.html), [*students create*](struct.CourseStudentCreateCall.html), [*students delete*](struct.CourseStudentDeleteCall.html), [*students get*](struct.CourseStudentGetCall.html), [*students list*](struct.CourseStudentListCall.html), [*teachers create*](struct.CourseTeacherCreateCall.html), [*teachers delete*](struct.CourseTeacherDeleteCall.html), [*teachers get*](struct.CourseTeacherGetCall.html), [*teachers list*](struct.CourseTeacherListCall.html) and [*update*](struct.CourseUpdateCall.html)
//! * [invitations](struct.Invitation.html)
//!  * [*accept*](struct.InvitationAcceptCall.html), [*create*](struct.InvitationCreateCall.html), [*delete*](struct.InvitationDeleteCall.html), [*get*](struct.InvitationGetCall.html) and [*list*](struct.InvitationListCall.html)
//! * [user profiles](struct.UserProfile.html)
//!  * [*get*](struct.UserProfileGetCall.html), [*guardian invitations create*](struct.UserProfileGuardianInvitationCreateCall.html), [*guardian invitations get*](struct.UserProfileGuardianInvitationGetCall.html), [*guardian invitations list*](struct.UserProfileGuardianInvitationListCall.html), [*guardian invitations patch*](struct.UserProfileGuardianInvitationPatchCall.html), [*guardians delete*](struct.UserProfileGuardianDeleteCall.html), [*guardians get*](struct.UserProfileGuardianGetCall.html) and [*guardians list*](struct.UserProfileGuardianListCall.html)
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
//! * **[Hub](struct.Classroom.html)**
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
//! let r = hub.courses().course_work_student_submissions_patch(...).doit()
//! let r = hub.courses().course_work_student_submissions_list(...).doit()
//! let r = hub.courses().get(...).doit()
//! let r = hub.courses().update(...).doit()
//! let r = hub.courses().students_delete(...).doit()
//! let r = hub.courses().teachers_get(...).doit()
//! let r = hub.courses().course_work_list(...).doit()
//! let r = hub.courses().teachers_list(...).doit()
//! let r = hub.courses().course_work_student_submissions_turn_in(...).doit()
//! let r = hub.courses().course_work_student_submissions_modify_attachments(...).doit()
//! let r = hub.courses().course_work_student_submissions_return(...).doit()
//! let r = hub.courses().course_work_get(...).doit()
//! let r = hub.courses().course_work_create(...).doit()
//! let r = hub.courses().list(...).doit()
//! let r = hub.courses().course_work_student_submissions_reclaim(...).doit()
//! let r = hub.courses().aliases_create(...).doit()
//! let r = hub.courses().students_create(...).doit()
//! let r = hub.courses().aliases_delete(...).doit()
//! let r = hub.courses().create(...).doit()
//! let r = hub.courses().students_list(...).doit()
//! let r = hub.courses().delete(...).doit()
//! let r = hub.courses().patch(...).doit()
//! let r = hub.courses().aliases_list(...).doit()
//! let r = hub.courses().teachers_delete(...).doit()
//! let r = hub.courses().teachers_create(...).doit()
//! let r = hub.courses().course_work_student_submissions_get(...).doit()
//! let r = hub.courses().students_get(...).doit()
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
//! google-classroom1 = "*"
//! ```
//! 
//! ## A complete example
//! 
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate yup_oauth2 as oauth2;
//! extern crate google_classroom1 as classroom1;
//! use classroom1::{Result, Error};
//! # #[test] fn egal() {
//! use std::default::Default;
//! use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
//! use classroom1::Classroom;
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
//! let mut hub = Classroom::new(hyper::Client::new(), auth);
//! // You can configure optional parameters by calling the respective setters at will, and
//! // execute the final call using `doit()`.
//! // Values shown here are possibly random and not representative !
//! let result = hub.courses().course_work_student_submissions_list("courseId", "courseWorkId")
//!              .user_id("gubergren")
//!              .add_states("aliquyam")
//!              .page_token("eos")
//!              .page_size(-38)
//!              .late("sea")
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