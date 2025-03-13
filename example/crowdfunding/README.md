### This contract is designed to gather a specific amount of funds and send them to a designated recipient. If the total fund does not meet the predefined target by a certain timeframe, the collected amount will be returned to the investors.

**How to use:**

Contract Creator

1. The contract creator calls init() to initialize the crowdfunding contract.

Users

1. Users call contribute() to participate in the crowdfunding by contributing funds.

Supercontract

1. When the specified block height is reached and the funding goal is met, the funds will be automatically sent to the funder. Otherwise, the funds will be returned to the investors.

#### ```init()```
This function is used during the deployment of the contract. The contract caller is required to specify the target amount for crowdfunding in the call parameters.

#### ```contribute()```
This function allows participants to contribute to the crowdfunding campaign. The contract caller must provide the investment amount in the service payment field, and the caller's address in the call parameters.

#### ```distribute()```
This function handles the distribution of funds to the recipient. It should be executed with the autorun feature to ensure automatic distribution or return of the funds to investors at a specified block height.
