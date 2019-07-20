salary_cap: int

category_counts: [int], list of integers where an integer represents
                        the count of items for that bucket, or list index.

player_pool: [players<double>], list of items where each item has
                                a value (projected points), a weight (salary cost),
                                and a list of categories the items falls under.


Algorithm description:
        Keep a cache of items we have selected. The key is a tuple marking the number
        of items, weight, and category_counts values from when the items was inserted
        into the cache. This will hereby be refered to as the 'key tuple'.
        The value is a tuple of the cumulative value, whether the aggregation
        is value, and the set of unique items collected. This will hereby, be referred
        to as the 'value tuple'
        
        Quick note, the goal is to maximize the cumulative value while maintaining validity
        and a set of non-duplicated items leading to the value.

        We define a recursive lambda function which returns value tuple, and takes the
        items inside the key tuple. It has a reference to the item list from the parent
        function.

        First, check if the cache already has a value for the current key tuple equivalent.
        If so, just return the value in the cache. Revisits a common ancestor point in the
        recursion tree (woot!).

        Then perform some basic validation on item count and category count values

        Then, loop through each remaining item and keep a tally of each category. Check if
        there are more categories needed than we have left in the item list. If so, create
        and return an invalid value tuple.
        
        Now, take the last item in the list (n - 1). The relavent pieces are the value (projected_points),
        weight (price), categories (currently just grabbing the first one), and the name (to maintain uniqueness,
        should probably use ID or something else since names aren't guaranteed to be unique). Check the weight
        to make sure this item will fit in the knapsack. Also check to make sure the category fits into our 
        category count totals. If either of these checks fails, just recurse to the next item, not taking the current.
        Subtract one from the category count and recurse assuming we have taken this
        value into our knapsack. Then add one back to the category count, to revert the previous operation and
        recurse assuming we do not take this item into our knapsack.
        We check the value and validity of the results of each of the two recursive operations.
        
        If each result is valid, we take the max value from the included and rejected recursive operations.
        In case that the include result is greater, check for a duplicated value (extraneous, we're using a set lmao),
        and insert. Form the next key and value tuples, inserting into the cache and
        returning the value tuple.
        
        If only the include result is valid, insert the item into the item set. Form the next key and value tuples,
        inserting into the cache and returning the value tuple. TODO: there is a bug in my implementation where
        if the insert returns false, it just recurses right from the failed check. Fix this to be like the
        block above.

        If only the rejected result is valid, just form the next key and value tuples using the rejected values.

        If neither result is valid, create a value tuple from (0, false, {}). Basically a dead end.

        If the initial weight and category check failed, recurse.


==============================================================================================================================


The above implementation doesn't process multiple categories for a single item. How to take multiple categories
into account? I have tried just wrapping the larger conditional tree with a for loop which loops through each
category for the current item. It would always return invalid result overall. Why didn't this work?