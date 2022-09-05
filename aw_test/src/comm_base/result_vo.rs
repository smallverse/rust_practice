use crate::comm_base::result_code;
use crate::comm_base::result_code::{
    CODE_ERROR, CODE_FAILED, CODE_SUCCESS, MSG_FAILED, MSG_SUCCESS,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResultVO<T> {
    /** 状态码，比如1000代表响应成功 */
    pub code: i32,
    /** 响应信息，用来说明响应情况 */
    pub msg: String,
    /** 响应的具体数据 */
    pub data: T,
}

impl<T> ResultVO<T> {
    pub fn gen_result(code: i32, msg: String, data: T) -> ResultVO<T> {
        ResultVO { code, msg, data }
    }

    pub fn gen_success_result(data: T) -> ResultVO<T> {
        ResultVO {
            code: *CODE_SUCCESS,
            msg: MSG_SUCCESS.to_string(),
            data,
        }
    }

    pub fn gen_failed_result(data: T) -> ResultVO<T> {
        ResultVO {
            code: *CODE_FAILED,
            msg: MSG_FAILED.to_string(),
            data,
        }
    }

    pub fn gen_error_result(data: T) -> ResultVO<T> {
        ResultVO {
            code: *CODE_ERROR,
            msg: CODE_ERROR.to_string(),
            data,
        }
    }
}
