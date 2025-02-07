### Seller deposit their asset to the contract and write how much he wish to sell. Buyer send the predefined specific token id and amount to the contract. If the conditions are fulfilled, asset is send to buyer, token is send to seller by the Supercontract.

#### ```deposit_asset()```
This function is used when deploying the contract. Contract caller is required to input the asset he wish to sell in service payment, and amount he wish to get in call parameter.

#### ```buy_asset()```
This function is to buy asset. Contract caller is required to input mosaic id and amount in the service payment and caller's address in call parameter.
