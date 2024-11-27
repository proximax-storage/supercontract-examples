### This contract will stake participant's fund.
#### ```deposit_interest()```
Contract creator is required to deposit some xpx into the Supercontract. this is used to pay staker's interest. When calling this function, caller should input amount to deposit into Supercontract in the service payment column.
#### ```join()```
Minimum amount to stake is 100xpx, to join the stake, caller should input their xpx in the service payment column and their address in call parameter. 
#### ```withdraw()```
To withdraw the staked token, user is required to call the withdraw function. If the stake duration is met, the stake amount + 10% interest are return to the user. When calling the function user is required to input thier address in the call parameter.


