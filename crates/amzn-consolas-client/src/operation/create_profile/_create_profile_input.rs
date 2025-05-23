// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateProfileInput {
    #[allow(missing_docs)] // documentation missing in model
    pub identity_source: ::std::option::Option<crate::types::IdentitySource>,
    #[allow(missing_docs)] // documentation missing in model
    pub profile_name: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub description: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub reference_tracker_configuration: ::std::option::Option<crate::types::ReferenceTrackerConfiguration>,
    #[allow(missing_docs)] // documentation missing in model
    pub active_functionalities: ::std::option::Option<::std::vec::Vec<crate::types::FunctionalityName>>,
    #[allow(missing_docs)] // documentation missing in model
    pub client_token: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub kms_key_arn: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    #[allow(missing_docs)] // documentation missing in model
    pub resource_policy: ::std::option::Option<crate::types::ResourcePolicy>,
    #[allow(missing_docs)] // documentation missing in model
    pub opt_in_features: ::std::option::Option<crate::types::OptInFeatures>,
}
impl CreateProfileInput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn identity_source(&self) -> ::std::option::Option<&crate::types::IdentitySource> {
        self.identity_source.as_ref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn profile_name(&self) -> ::std::option::Option<&str> {
        self.profile_name.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn reference_tracker_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::ReferenceTrackerConfiguration> {
        self.reference_tracker_configuration.as_ref()
    }

    #[allow(missing_docs)] // documentation missing in model
    /// If no value was sent for this field, a default will be set. If you want to determine if no
    /// value was sent, use `.active_functionalities.is_none()`.
    pub fn active_functionalities(&self) -> &[crate::types::FunctionalityName] {
        self.active_functionalities.as_deref().unwrap_or_default()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn kms_key_arn(&self) -> ::std::option::Option<&str> {
        self.kms_key_arn.as_deref()
    }

    #[allow(missing_docs)] // documentation missing in model
    /// If no value was sent for this field, a default will be set. If you want to determine if no
    /// value was sent, use `.tags.is_none()`.
    pub fn tags(&self) -> &[crate::types::Tag] {
        self.tags.as_deref().unwrap_or_default()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn resource_policy(&self) -> ::std::option::Option<&crate::types::ResourcePolicy> {
        self.resource_policy.as_ref()
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn opt_in_features(&self) -> ::std::option::Option<&crate::types::OptInFeatures> {
        self.opt_in_features.as_ref()
    }
}
impl CreateProfileInput {
    /// Creates a new builder-style object to manufacture
    /// [`CreateProfileInput`](crate::operation::create_profile::CreateProfileInput).
    pub fn builder() -> crate::operation::create_profile::builders::CreateProfileInputBuilder {
        crate::operation::create_profile::builders::CreateProfileInputBuilder::default()
    }
}

/// A builder for [`CreateProfileInput`](crate::operation::create_profile::CreateProfileInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CreateProfileInputBuilder {
    pub(crate) identity_source: ::std::option::Option<crate::types::IdentitySource>,
    pub(crate) profile_name: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) reference_tracker_configuration: ::std::option::Option<crate::types::ReferenceTrackerConfiguration>,
    pub(crate) active_functionalities: ::std::option::Option<::std::vec::Vec<crate::types::FunctionalityName>>,
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) kms_key_arn: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    pub(crate) resource_policy: ::std::option::Option<crate::types::ResourcePolicy>,
    pub(crate) opt_in_features: ::std::option::Option<crate::types::OptInFeatures>,
}
impl CreateProfileInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn identity_source(mut self, input: crate::types::IdentitySource) -> Self {
        self.identity_source = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_identity_source(mut self, input: ::std::option::Option<crate::types::IdentitySource>) -> Self {
        self.identity_source = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_identity_source(&self) -> &::std::option::Option<crate::types::IdentitySource> {
        &self.identity_source
    }

    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn profile_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.profile_name = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_profile_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.profile_name = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_profile_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.profile_name
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }

    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn reference_tracker_configuration(mut self, input: crate::types::ReferenceTrackerConfiguration) -> Self {
        self.reference_tracker_configuration = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_reference_tracker_configuration(
        mut self,
        input: ::std::option::Option<crate::types::ReferenceTrackerConfiguration>,
    ) -> Self {
        self.reference_tracker_configuration = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_reference_tracker_configuration(
        &self,
    ) -> &::std::option::Option<crate::types::ReferenceTrackerConfiguration> {
        &self.reference_tracker_configuration
    }

    /// Appends an item to `active_functionalities`.
    ///
    /// To override the contents of this collection use
    /// [`set_active_functionalities`](Self::set_active_functionalities).
    pub fn active_functionalities(mut self, input: crate::types::FunctionalityName) -> Self {
        let mut v = self.active_functionalities.unwrap_or_default();
        v.push(input);
        self.active_functionalities = ::std::option::Option::Some(v);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_active_functionalities(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::FunctionalityName>>,
    ) -> Self {
        self.active_functionalities = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_active_functionalities(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<crate::types::FunctionalityName>> {
        &self.active_functionalities
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_token
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn kms_key_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.kms_key_arn = ::std::option::Option::Some(input.into());
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_kms_key_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.kms_key_arn = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_kms_key_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.kms_key_arn
    }

    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.tags = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        &self.tags
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn resource_policy(mut self, input: crate::types::ResourcePolicy) -> Self {
        self.resource_policy = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_resource_policy(mut self, input: ::std::option::Option<crate::types::ResourcePolicy>) -> Self {
        self.resource_policy = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_resource_policy(&self) -> &::std::option::Option<crate::types::ResourcePolicy> {
        &self.resource_policy
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn opt_in_features(mut self, input: crate::types::OptInFeatures) -> Self {
        self.opt_in_features = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_opt_in_features(mut self, input: ::std::option::Option<crate::types::OptInFeatures>) -> Self {
        self.opt_in_features = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_opt_in_features(&self) -> &::std::option::Option<crate::types::OptInFeatures> {
        &self.opt_in_features
    }

    /// Consumes the builder and constructs a
    /// [`CreateProfileInput`](crate::operation::create_profile::CreateProfileInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_profile::CreateProfileInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_profile::CreateProfileInput {
            identity_source: self.identity_source,
            profile_name: self.profile_name,
            description: self.description,
            reference_tracker_configuration: self.reference_tracker_configuration,
            active_functionalities: self.active_functionalities,
            client_token: self.client_token,
            kms_key_arn: self.kms_key_arn,
            tags: self.tags,
            resource_policy: self.resource_policy,
            opt_in_features: self.opt_in_features,
        })
    }
}
