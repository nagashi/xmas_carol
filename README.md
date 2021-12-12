<center><h1>The 12 Days of Christmas</h1></center>
<p align="center"> 
        <img src = "images/Twelve_Smurfs_Of_Christmas.gif" alt="Image denoting the 12 Smurfs of Christmas"
          width="225"
          height="175"
          border="3"
        />
</p>
<br>
<details>
  <summary>&ensp;Click to view explanation of this program </summary><p></p>
  
  &ensp;Programatically, The 12 Days of Christmas has been constructed in different languages in multiple ways. Each day begins with the line _"On the {} day of Christmas my true love sent to me:"._&ensp;To handle this, within the for loop of the main function, each of the inclusive 12 days is being passed to the gen_verse function.&ensp;The first line within gen_verse formats the day/number argument as an ordinal number, i.e. (1st, 2nd, 3rd, 4th, ...) and assigns the new value to a variable.&ensp;Next we work with the one constant phrase in the lyric <em>"On the {} day of Christmas, my true love gave to me"</em> and converts this phrase along with the ordinal variable into a string using the format! macro, which again, is assigned to a variable.
  <br> <br>
  <b><li>The constant array listed in descending order.</li></b><br> 
  <b><li>The skip method.</b></li>
</ol>
<p></p></details>
<br>
&ensp;The gen_verse function is called within the for loop of the main function with each of the inclusive 12 days being passed in as an argument.&ensp;The first line within gen_verse formats the day/number argument as an ordinal number, i.e. (1st, 2nd, 3rd, 4th, ...) and assigns the new value to a variable.&ensp;Next we work with the one constant phrase in the lyric <em>"On the {} day of Christmas, my true love gave to me"</em> and convert the phrase along with the ordinal variable into a string using the format! macro, which again, is assigned to a variable. The final piece of the function deals with displaying the Christmas presents.  Remember that the constant array housing the gifts was constructed in decending order.  The array is iterated over after skipping 'n' items from the first item in the array, which is "Twelve drummers drumming" to the 11th item "Two turtle doves, and". Each subsequent iteration with a new day value displays the appropriate lyrics for that appropriate day.  View the output in the image below.
<div align="center">12 Days of Christmas output</div>
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



<br><br><br>

<details>
<summary> Play Music</summary>
<video  width = "300" height = "200" controls autoplay>
         <source src = "https://static.wikia.nocookie.net/smurfsfanon/images/6/68/Smurfs_-_The_Twelve_Smurfs_Of_Christmas.ogg/revision/latest?cb=20131220194922"" type ="video/ogg" />
         <source src = "/html5/foo.mp4" type = "video/mp4" />
         Your browser does not support the <video> element.
      </video>



<div class="mediaContainer" style="width:18px"><audio id="mwe_player_0" controls="autoplay" preload="auto" style="width:180px" class="kskin" data-durationhint="222.41569160998" data-startoffset="0" data-mwtitle="Smurfs_-_The_Twelve_Smurfs_Of_Christmas.ogg" data-mwprovider="local"><source src="https://static.wikia.nocookie.net/smurfsfanon/images/6/68/Smurfs_-_The_Twelve_Smurfs_Of_Christmas.ogg/revision/latest?cb=20131220194922" type="audio/ogg; codecs=&quot;vorbis&quot;" data-title="Original Ogg file (159 kbps)" data-shorttitle="Ogg source" data-width="0" data-height="0" data-bandwidth="158851" /></audio></div>
</summary>
</details>


<br><br><br><br>


<button type="button" onclick="handleBtnClick(event)" onKeyDown="handleBtnKeyDown(event)">
  Play Audio
  <audio id="audio" src="https://soundbible.com/mp3/Tyrannosaurus%20Rex%20Roar-SoundBible.com-807702404.mp3">
  
</audio>
</button>

<span role="button" tabindex="0"
 aria-pressed="true" onclick="handleBtnClick(event)"
 onKeyDown="handleBtnKeyDown(event)">
  Play Audio
</span>

<audio id="audio" src="https://soundbible.com/mp3/Tyrannosaurus%20Rex%20Roar-SoundBible.com-807702404.mp3">
  
</audio>

<br><br>
