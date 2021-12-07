 <p align="center">
        <kbd><img src = "images/Twelve_Smurfs_Of_Christmas.gif" alt="Image denoting the 12 Smurfs of Christmas"
          width="225"
          height="175"
          border="3"
          borderColor="red"
        /></kbd>
</p>
<br>
There are three factors that contribute to the algorithm to make this program work:
<ol type="I">
  <li> The constant array listed in descending order.</li> 
  <li> The skip method on Iterator.</li> 
  <li> The inclusive range in the for loop.</li>
</ol>
<br>
The gen_verse function is called within the for loop with each 'day' being passed to it as an argument.  The first line within the function formats the day/number argument as an ordinal number (1st, 2nd, 3rd, 4th, ...).  Next we work with the one constant phrase in the lyric <em>"On the {} day of Christmas, my true love gave to me"</em> and convert the phrase along with the ordinal value into a string using the format! macro. The final piece of the function deals with displaying the Christmas presents.  Remember that the constant array housing the gifts was constructed in decending order.  The array is iterated over after skipping 'n' items. The remaining items are read and displayed.  It is the utilization of the skip function along with the way the array is constructed that makes this program work. 

<p align="center">
        <kbd><img src = "images/output.gif" alt="Image denoting the program output"
          width="225"
          height="175"
          border="3"
          borderColor="red"
        /></kbd>
</p>
<br>