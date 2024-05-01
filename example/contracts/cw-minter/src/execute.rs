use crate::{
    contract::Contract,
    error::{ContractError, ContractResponse},
    events::{
        ACTION, CW721_SET_EVENT, MINTED_TOKEN_ID_EVENT, MINTED_TOKEN_OWNER_EVENT, MINT_ACTION,
    },
    msg::{MintMsg, SetCw721Msg},
    utils::{
        get_weighted_option::get_weighted_option,
        rand::rand,
        validators::{parse_address, validate_funds},
    },
};
use cosmwasm_std::{to_json_binary, DepsMut, Env, MessageInfo, Response, WasmMsg};
use cw721_base::{ExecuteMsg as Cw721ExecuteMsg, MintMsg as Cw721MintMsg};
use serde::{de::DeserializeOwned, Serialize};

impl<'a, TExtension> Contract<'a, TExtension>
where
    TExtension: Serialize + DeserializeOwned + Clone,
{
    /// Mint a token
    pub fn mint(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        _msg: MintMsg,
    ) -> ContractResponse {
        let config = self.require_instantiated(&deps.as_ref(), &info)?;
        let cw721 = config.cw721.ok_or(ContractError::NoCw721ContractSet {})?;

        let mint_count = self.mint_count.load(deps.storage)?;

        // If supply is set, check current amount of minted tokens against it
        if let Some(supply) = config.supply {
            if mint_count >= supply {
                return Err(ContractError::MintedOut { supply });
            }
        }

        // If mint price is set, check the attached funds
        if let Some(price) = config.price {
            validate_funds(&info.funds, &price)?;
        }

        // Update mint count
        let new_mint_count = mint_count + 1;
        self.mint_count.save(deps.storage, &new_mint_count)?;

        // Get the extension
        // FIXME: get *random* number, not just hash
        // @see https://docs.nois.network/dapp_devs/contract_implementation.html
        let rand = rand(env.block.height, env.block.time.nanos(), new_mint_count);
        let extension = get_weighted_option(rand, &config.extensions).clone();

        // Mint the token
        let token_id = new_mint_count.to_string();
        let mint_msg = Cw721ExecuteMsg::Mint(Cw721MintMsg::<TExtension> {
            token_id: token_id.clone(),
            owner: info.sender.clone().into(),
            token_uri: None,
            extension,
        });
        let mint_resp = WasmMsg::Execute {
            contract_addr: cw721.into(),
            msg: to_json_binary(&mint_msg)?,
            funds: vec![],
        };

        Ok(Response::new()
            .add_attribute(ACTION, MINT_ACTION)
            .add_attribute(MINTED_TOKEN_ID_EVENT, token_id)
            .add_attribute(MINTED_TOKEN_OWNER_EVENT, info.sender.to_string())
            .add_message(mint_resp))
    }

    /// Set cw721 contract
    pub fn set_cw721(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: SetCw721Msg,
    ) -> ContractResponse {
        let mut config = self.require_instantiated(&deps.as_ref(), &info)?;
        self.require_admin(&deps.as_ref(), &info)?;

        let cw721 = parse_address(&msg.address, &deps.as_ref())?;

        config.cw721 = Some(cw721.clone());
        self.config.save(deps.storage, &config)?;

        Ok(Response::new().add_attribute(CW721_SET_EVENT, cw721))
    }
}
