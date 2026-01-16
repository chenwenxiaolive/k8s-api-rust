use super::*;

impl InternalConversion for APIService {
    type Internal = crate::apiregistration::internal::APIService;
}

impl InternalConversion for APIServiceList {
    type Internal = crate::apiregistration::internal::APIServiceList;
}

impl InternalConversion for APIServiceSpec {
    type Internal = crate::apiregistration::internal::APIServiceSpec;
}

impl InternalConversion for APIServiceStatus {
    type Internal = crate::apiregistration::internal::APIServiceStatus;
}

impl InternalConversion for ServiceReference {
    type Internal = crate::apiregistration::internal::ServiceReference;
}
