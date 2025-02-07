### Only 20 players can join each game. To join the game, player are required to buy a ticket(100xpx). A winner will pick at random among the 20 player. 95% of the token to the winner, 5% of the token are kept in the Supercontract account

#### ```init()```
This function is used when deploying the contract. It create neccesary file to run the contract.

#### ```join()```
This function is to join to the lottery. Contract caller is required to input 100xpx in the service payment column and account address in the call parameter.

#### ```distribute()```
This function is to send the prize to the winner. It should be call with autorun feature so that it will automatically send the prize or return the ticket amount to player if there if not enough 20 players join the game.
