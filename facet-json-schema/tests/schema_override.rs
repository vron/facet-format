use facet::Facet;
use facet_json_schema::{SchemaType, SchemaTypes, schema_for};

#[derive(Facet)]
#[facet(facet_json_schema::unconstrained_string)]
/// One of the values supported by the application.
#[repr(u8)]
enum LargeEnum {
    Alpha,
    Beta,
    Gamma,
}

#[derive(Facet)]
struct Request {
    value: LargeEnum,
}

#[test]
fn enum_schema_can_be_an_unconstrained_string() {
    let schema = schema_for::<LargeEnum>();
    assert!(matches!(
        schema.type_,
        Some(SchemaTypes::Single(SchemaType::String))
    ));
    assert!(schema.enum_.is_none());
    assert_eq!(schema.title.as_deref(), Some("LargeEnum"));
    assert_eq!(
        schema.description.as_deref(),
        Some("One of the values supported by the application.")
    );
}

#[test]
fn override_is_used_when_the_enum_is_nested() {
    let schema = schema_for::<Request>();
    let value = schema
        .properties
        .as_ref()
        .and_then(|properties| properties.get("value"))
        .expect("value property");
    assert!(matches!(
        value.type_,
        Some(SchemaTypes::Single(SchemaType::String))
    ));
    assert!(value.enum_.is_none());
    assert_eq!(
        value.description.as_deref(),
        Some("One of the values supported by the application.")
    );
}

#[test]
fn override_does_not_change_json_serialization_or_deserialization() {
    assert_eq!(
        facet_json::to_string(&LargeEnum::Beta).unwrap(),
        r#""Beta""#
    );
    assert!(facet_json::from_str::<LargeEnum>(r#""Unknown""#).is_err());
}
