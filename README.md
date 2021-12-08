 <p align="center">
        <kbd><img src = "images/Twelve_Smurfs_Of_Christmas.gif" alt="Image denoting the 12 Smurfs of Christmas"
          width="225"
          height="175"
          border="3"
          borderColor="red"
        /></kbd>
</p>
<br>
<details>
  <summary>Click to view factors which enable this program to work: </summary></p>
  <ol type="I">
  <li> The constant array listed in descending order.</li> 
  <li> The skip method on Iterator.</li> 
  <li> The inclusive range in the for loop.</li>
</ol>
</pre></details>
<br>
The gen_verse function is called within the for loop of the main function with each of the 12 days being passed to it as an argument.  The first line within gen_verse formats the day/number argument as an ordinal number (1st, 2nd, 3rd, 4th, ...).  Next we work with the one constant phrase in the lyric <em>"On the {} day of Christmas, my true love gave to me"</em> and convert the phrase along with the ordinal value into a string using the format! macro. The final piece of the function deals with displaying the Christmas presents.  Remember that the constant array housing the gifts was constructed in decending order.  The array is iterated over after skipping 'n' items from the first item in the array, which is "Twelve drummers drumming" to the 11th item "Two turtle doves, and". Each subsequent iteration with a new day value displays the appropriate lyrics for that appropriate day.  View the output in the image below.

<p align="center">
        <kbd><img src = "images/output.gif" alt="Image denoting the program output"
          width="225"
          height="175"
          border="3"
          borderColor="red"
        /></kbd>
</p>
<br>

** accordion code**
<details>
  <summary>Click to view factors </summary><p></p>
<p></p><pre>
  * The constant array listed in descending order.
  * The skip method on Iterator.
  * The inclusive range in the for loop.
  <p></p>
</pre></details>