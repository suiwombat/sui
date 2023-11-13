// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies the inventory configuration for an Amazon S3 bucket.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InventoryDestination {
    /// <p>Contains the bucket name, file format, bucket owner (optional), and prefix (optional) where inventory results are published.</p>
    pub s3_bucket_destination: ::std::option::Option<crate::types::InventoryS3BucketDestination>,
}
impl InventoryDestination {
    /// <p>Contains the bucket name, file format, bucket owner (optional), and prefix (optional) where inventory results are published.</p>
    pub fn s3_bucket_destination(&self) -> ::std::option::Option<&crate::types::InventoryS3BucketDestination> {
        self.s3_bucket_destination.as_ref()
    }
}
impl InventoryDestination {
    /// Creates a new builder-style object to manufacture [`InventoryDestination`](crate::types::InventoryDestination).
    pub fn builder() -> crate::types::builders::InventoryDestinationBuilder {
        crate::types::builders::InventoryDestinationBuilder::default()
    }
}

/// A builder for [`InventoryDestination`](crate::types::InventoryDestination).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct InventoryDestinationBuilder {
    pub(crate) s3_bucket_destination: ::std::option::Option<crate::types::InventoryS3BucketDestination>,
}
impl InventoryDestinationBuilder {
    /// <p>Contains the bucket name, file format, bucket owner (optional), and prefix (optional) where inventory results are published.</p>
    pub fn s3_bucket_destination(mut self, input: crate::types::InventoryS3BucketDestination) -> Self {
        self.s3_bucket_destination = ::std::option::Option::Some(input);
        self
    }
    /// <p>Contains the bucket name, file format, bucket owner (optional), and prefix (optional) where inventory results are published.</p>
    pub fn set_s3_bucket_destination(mut self, input: ::std::option::Option<crate::types::InventoryS3BucketDestination>) -> Self {
        self.s3_bucket_destination = input;
        self
    }
    /// <p>Contains the bucket name, file format, bucket owner (optional), and prefix (optional) where inventory results are published.</p>
    pub fn get_s3_bucket_destination(&self) -> &::std::option::Option<crate::types::InventoryS3BucketDestination> {
        &self.s3_bucket_destination
    }
    /// Consumes the builder and constructs a [`InventoryDestination`](crate::types::InventoryDestination).
    pub fn build(self) -> crate::types::InventoryDestination {
        crate::types::InventoryDestination {
            s3_bucket_destination: self.s3_bucket_destination,
        }
    }
}
