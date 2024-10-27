use alloy::{
    primitives::{Address, U256},
    providers::Provider,
    sol,
};

use eyre::Result;

use alloy::transports::Transport;

use crate::erc20::IERC20::IERC20Instance;


// ERC 20 contract specifying return value names
sol! {
    #[sol(rpc)]    
    interface IERC20 {
        function totalSupply() external view returns (uint256 supply);
        function balanceOf(address account) external view returns (uint256 balance);
        function transfer(address recipient, uint256 amount) external returns (bool success);
        function allowance(address owner, address spender) external view returns (uint256 allowance);
        function approve(address spender, uint256 amount) external returns (bool success);
        function transferFrom(address sender, address recipient, uint256 amount) external returns (bool success);
        event Transfer(address indexed from, address indexed to, uint256 value);
        event Approval(address indexed owner, address indexed spender, uint256 value);

        function symbol() external view returns (string sym);
    }

}

pub struct Erc20<T, P> {
    pub token_contract: IERC20Instance<T, P>,
}

impl<T, P> Erc20<T, P> where 
    T: Transport + Clone,
    P: Provider<T> + Clone,
{
//    pub async fn new<T, P>(
    pub async fn new(
        token_address: Address,
        provider: P,
    ) -> Result<Self>
    where
        T: Transport + Clone,
        P: Provider<T> + Clone,
    {
        let token_contract = IERC20::new(token_address, provider);
        let tok0_symbol = token_contract.symbol().call().await?.sym;
        println!("Sym: {}", tok0_symbol);

        Ok(Self{token_contract})
    }

    pub async fn address(&self) -> Result<&Address> {
        let addr = self.token_contract.address();
        Ok(addr)
    }

    pub async fn symbol(&self) -> Result<String> {
        let tok0_symbol = self.token_contract.symbol().call().await?.sym;
        Ok(tok0_symbol)
    }

    pub async fn total_supply(&self) -> Result<U256> {
        let res = self.token_contract.totalSupply().call().await?.supply;
        Ok(res)
    }

}