### This contract gathers specific amount of fund and send to recipient. If the fund amount does not meet the predefined value at certain timeframe, the fund will be return to the investor.

#### ```init()```
This function is used when deploying the contract. Contract caller is required to input amount to crowdfund in call parameter.

#### ```contribute()```
This function is to contribute to the crowdfunding. Contract caller is required to input amount to invest in the service payment column and caller's address in the call parameter.

#### ```distribute()```
This function is to send the fund to the recipient. It should be call with autorun feature so that it will automatically send the fund or return the fund to investor at specific block height.
