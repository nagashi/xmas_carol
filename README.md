<h1 align="center">The 12 Days of Christmas
  <h3 align="center">Written in Rust</h3>
</h1>
<p align="center"> 
        <img src = "images/Twelve_Smurfs_Of_Christmas.gif" alt="Image denoting the 12 Smurfs of Christmas"
          width="225"
          height="175"
          border="3"
        />
</p>

<br>
<details>
  <summary align="center">&ensp;Click <i><u>here</u></i> to view a description of this program </summary><p></p>
  
  &ensp;&ensp;Programatically, "The 12 Days of Christmas" has been constructed in different programming languages and in multiple ways.&ensp;&ensp;What follows is my rendition as written using the Rust language.
  <br>
  <br>
  &ensp;&ensp;Within the for loop of the main function, each of the inclusive 12 days is being passed to the gen_verse function.&ensp;The first line within gen_verse formats the day/number argument as an ordinal number, i.e. (1st, 2nd, 3rd, 4th, ...) and assigns the value to the variable 'ordinal_suffix'.&ensp; &ensp;Next, work is performed with the one consistent phrase in the lyric _"On the {} day of Christmas my true love sent to me:"._&ensp;&ensp;This phrase is converted, along with the ordinal_suffix variable, into a string using the format! macro, which is assigned to the variable 'begin'.
  <br>
  <br>
  &ensp;&ensp;The final piece of the function deals with displaying the Christmas presents.&ensp;&ensp;A constant array housing the gifts is constructed in decending order.&ensp;&ensp;After skipping '12 - n' items in the array, beginning with the first item, which is "Twelve drummers drumming", the remaining items are iterated over for that day and any previous day(s).&ensp;&ensp;For each iteration, a new line character is appended and then the lyric is appended to the 'begin' variable.&ensp;&ensp;At the completion of that days' iteration, the 'begin' variable is returned from the gen_verse function to the calling function and printed.&ensp;&ensp;This process is completed for each 'day' value passed to gen_verse.&ensp;&ensp;The output can be viewed in the image below.
  <br>
  <br>
  Thanks for reading and do reach out and let me know if you have any questions or concerns.&nbsp;&nbsp;Click 'Star' if you like the program.&nbsp;&nbsp;All suggestions, constructive, even non-constructive, will be welcomed.<img src = "images/ok.png" alt="Image denoting Ok"
          width="30"
          height="20"
          border="0"
        />&nbsp;&nbsp;The source code can be viewed in the [src/main](https://github.com/nagashi/xmas_carol/blob/main/src/main.rs) folder for those unfamiliar with the Rust language.
 </details>  
<br>
<div align="center">12 Days of Christmas output</div>
<p align="center">
        <img src = "images/output.gif" alt="Image denoting the program output"
          width="225"
          height="175"
          border="3"
        />
</p>
<div align="center"> 

[![MIT licensed][mit-badge]][mit-url]&nbsp;&nbsp;[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)

</div>
<br>
<br>
<p>

### License

This project is licensed under the&nbsp;[MIT license](LICENSE).

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE
### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Tracing by you, shall be licensed as MIT, without any additional
terms or conditions.

</p>
