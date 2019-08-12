Attempt to translate draftfast linear problem to lp-modeler and rust style.

Steps:

1) Define the variables
    Each player represents a variable with the projected points as the coefficient:
        Projected_Points * Player;
    

2) Define the objective and objective function
    
    Main objective function would look something like:
    proj_pts * player1 + proj_pts * player_2 + ...
    
    Maximize this function.

3) Define the constraints
    
    How to define the constraints for this scenario?
    What are the constraints?:
        - "Lineup constraints", meaning only 1 from QB group, 2 from RB group, 3 from WR group, 
           1 from TE group, 1 from FLEX group, 1 from DST group
           In the 'matchmaking example', the constraint is that one woman must be paired with exactly
           one man. To do this we create a constraint like:
           1.0 * var1 + 1.0 * var2 + 1.0*var3... = 1.0, where the var's are variable values which contain
           the current woman. 
           
           Using binary variables...
           I would need to place players into groups (QB, RB, WR, FLEX, DST). And do this for each group:
           1.0 * group1_player1 + 1.0 * group1_player2 + ... = 1.0; // for QBs
           1.0 * group2_player1 + 1.0 * group2_player2 + ... = 2.0; // for RBs
           1.0 * group3_player1 + 1.0 * group3_player2 + ... = 3.0  // for WRs
           ... for each group in the lineup.

           Also would need to make sure that players which belong to multiple groups are not included
           more that once. Do this by:
           1.0 * player <= 1.0;
           Would this work for showdown slates? See what draftfast did in this regard.
           I could build a third constraint set:
           1.0 * cpt_player + 1.0 * player <= 1

================================================

