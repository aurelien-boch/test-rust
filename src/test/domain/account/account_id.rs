use crate::domain::account::AccountId;
use crate::domain::services::id_generation::Id;

#[test]
fn from_should_create_aggregate_with_correct_value()
{
    let id = Id {
        value: "my_id".to_string()
    };

    let account_id = AccountId::from(id.clone());

    assert_eq!(account_id.value, format!("account_{}", id.value));
}
