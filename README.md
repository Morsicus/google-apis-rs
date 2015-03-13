[![Build Status](https://travis-ci.org/Byron/google-apis-rs.svg?branch=master)](https://travis-ci.org/Byron/google-apis-rs)

# TODO

* Adjust readme to reflect what this project actually is.

# Old youtube-specific readme

*The following is the previous youtube-specific writing, will have to think about how and if that should be integrated into the generated documentation.*

*Youtube* is a library written in Rust to help interacting with your youtube account.
For now, all functionality is geared towards allowing interruptible video uploads
and adjustments of video meta-data.

The library works using the builder pattern. If builders are instantiated, you will need to 
provide the minimal information right off the bat. Further calls to the builder allow 
to configure it. Some configuration will be in the form of callbacks, which allows you to 
control internal loops or behaviour.

It's the goal of each builder to maximize the chances of a successful result, and it will 
provide enough callbacks to be resilient against network errors, and authorization failures 
which require the token to be refreshed.

You will need authorization to perform most of the operations implemented here - it can be obtained
and handled using the [yup-oauth2 library][oauth].


# License

The license of everything not explicitly under a different license are licensed as specified in `LICENSE.md`.

What follows is a list of other material that is licensed differently.

* **./etc/api/\*\*/*.json** are licensed under a [MIT-like google license][google-lic].


[oauth]: https://crates.io/crates/yup-oauth2
[google-lic]: https://github.com/google/google-api-go-client/blob/master/LICENSE