use serde::{Serialize, Deserialize};
use uuid::Uuid;

type CreditIdDto = String;

type CreditCodeDto = u8;

#[derive(Serialize, Deserialize)]
pub struct CreateCreditRequestDto {
    pub id: CreditIdDto,
    pub code: CreditCodeDto
}

pub struct CreditId(Uuid);

impl CreditId {
    pub fn from_dto(dto: CreditIdDto) -> Result<CreditId, String> {
        match Uuid::parse_str(dto.as_str()) {
            Ok(val) => Ok(CreditId(val)),
            Err(error) => Err(error.to_string()),
        }
    }
}

pub struct CreditCode(u8);

impl CreditCode {
    pub fn from_dto(dto: CreditCodeDto) -> Result<CreditCode, String> {
        Ok(CreditCode(dto))
    }
}

pub struct CreateCreditRequest {
    id: CreditId,
    code: CreditCode
}

impl CreateCreditRequest {
    pub fn from_dto(dto: CreateCreditRequestDto) -> Result<CreateCreditRequest, String> {
        Ok(CreateCreditRequest {
            id: match CreditId::from_dto(dto.id) {
                Ok(val) => val,
                Err(str) => return Err(format!("credit id not valid {}", str))
            },
            code: match CreditCode::from_dto(dto.code) {
                Ok(val) => val,
                Err(str) => return Err(format!("credit code not valid {}", str))
            },
        })
    }
}
