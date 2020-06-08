# Web

## Compile to a target that runs natively on the web

WebAssembly.

### Compile C into WebAssembly (using Emscripten)

```shell
emcc some_source.c -s WASM=1 -o some-output.html
```

## Add structure

HTML.

### Use semantic HTML

Prefer standard semantic HTML over custom elements wherever possible since standard semantic HTML has the best accessibility, internationalization, performance, native integrations, etc.

### Create a fallback when scripting is not available

```html
<noscript>
  Some fallback element
</noscript>
```

### Create a section that holds the primary content

```html
<main>...</main>
```

### Create a section that holds content indirectly related to the primary content

```html
<aside>...</aside>
```

### Create a section that holds navigation links

```html
<nav>...</nav>
```

### Create a section that holds introductory content

```html
<header>...</header>
```

### Create a section that holds meta data content related to the section it is in

```html
<footer>...</footer>
```

### Create a section that holds a self-contained composition for things like syndication

```html
<article>...</article>
```

### Create a generic section that doesn't have a more specific semantic element

```html
<section>...</section>
```

### Create headings

```html
<h1>Some label</h1>
<h2>Some label</h2>
<h3>Some label</h3>
<h4>Some label</h4>
<h5>Some label</h5>
<h6>Some label</h6>
```

### Create a paragraph

```html
<p>Some content</p>
```

### Create a list of items

```html
<ul>
  <li>Some label</li>
  <li>Some label</li>
</ul>
```

### Create a list of items that are in order

```html
<ol>
  <li>Some label</li>
  <li>Some label</li>
</ol>
```

### Create a link to a URL

```html
<a href="some-url">Some label</a>
```

### Create clickable actions

```html
<button>Some label</button>
```

### Create interactive controls for submitting information

```html
<form>
  <label for="some-id">Some label</label>
  <input
    id="some-id"
    type="number/tel/url/email/password/file/color/radio/checkbox/range/text"
    ...
  />

  <label for="some-id">Some label</label>
  <textarea id="some-id" />

  <select>
    <option>Some label</option>
    <option>Some label</option>
  </select>

  <datalist>
    <option>Some label</option>
    <option>Some label</option>
  </datalist>

  <button type="submit">Some label</button>
</form>
```

The form elements can use client-side validation with validation attributes on form elements like `required`, `type`, `minlength`, `maxlength`, `min`, `max`, `pattern`.

### Create a date picker

```html
<input type="date" ... />
```

### Create an accordion

```html
<details>
  <summary>Some label</summary>
  ...
</details>
```

### Create a modal

```html
<dialog open>
  ...
</dialog>
```

### Create a progress indicator

```html
<label for="some-id">Some label</label>
<progress id="some-id" max="100" value="70">Some label</progress>
```

### Create an embedded image

```html
<img src="some-path" alt="Some fallback description" />
```

### Create an embedded image with lazy loading

```html
<img src="some-path" alt="Some fallback description" loading="lazy" />
```

### Create an embedded image with versions for different devices or screen sizes

```html
<picture>
  <source srcset="some-path" media="some-condition" />
  <source srcset="some-path" media="some-condition" />
  <img src="some-path" alt="Some fallback description" />
</picture>
```

### Create an embedded audio player

```html
<audio src="some-path" ...>
  Some fallback description
</audio>
```

### Create an embedded video player

```html
<video src="some-path" ...>
  Some fallback description
</video>
```

### Create an embedded web page

```html
<iframe src="some-path"></iframe>
```

### Create an embedded web page with lazy loading

```html
<iframe src="some-path" loading="lazy"></iframe>
```

### Create an embedded PDF

```html
<object type="application/pdf" data="some-path"></object>
```

### Create programmatic graphics

```html
<canvas>
  Some fallback description
</canvas>
```

### Create columns and rows of tabular data

```html
<table>
  <thead>
    <tr>
      <th>Some header</th>
      <th>Some header</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Some data</td>
      <td>Some data</td>
    </tr>
  </tbody>
</table>
```

### Create text with additional importance

```html
<strong>Some text</strong>
```

### Create text representing a specific time

```html
<time datetime="some-machine-readable-format">Some label</time>
```

### Create a quote

```html
<blockquote>Some quote</blockquote>
```

### Create self-contained content

```html
<figure>
  Some quote
  <figcaption>Some description</figcaption>
</figure>
```

### Create a forced line-break

```html
<br />
```

### Create text that contains programming code

```html
<code>some-code</code>
```

### Create text that retains the formatting used in the source

```html
<pre>some-text</pre>
```

### Create side comments

```html
<small>some-text</small>
```

## Add styles

CSS.

### Align items in a row vertically

```css
some-selector {
  display: flex;
  align-items: center;
}
```

### Add animation between changes

```css
a,
a:visited {
  color: LightSeaGreen;
  transition: 0.3s;
}

a:hover,
a:focus {
  opacity: 0.7;
}
```

## Add runtime logic

JavaScript.

### Create APIs that can be shared across files (modules)

```javascript
export default someThing
export anotherThing
```

### Use APIs that are shared across files (modules)

```javascript
import someThing, { anotherThing } from "some-path";
await import("some-path");
```

### Get meta data about a module

```javascript
import.meta;
```

### Create functions

```javascript
function someFunction(someParameter) { ... }
// or
(someParameter) => { ... }
```

### Use a default value for function parameters

```javascript
function someFunction(someParameter = someValue) { ... }
// or
(someParameter = someValue) => { ... }
```

### Apply a function on every item in an array

```javascript
someArray.map(someItem => // return updated item)
```

### Get items in an array that meet a condition

```javascript
someArray.filter(someItem => // return boolean for the condition)
```

### Check if all items in an array meet a condition

```javascript
someArray.every(someItem => // return boolean for the condition)
```

### Check if at least one item in an array meets a condition

```javascript
someArray.some(someItem => // return boolean for the condition)
```

### Convert an array to a single value

```javascript
someArray.reduce((someAccumulation, someItem)  => // return accumulation, someInitialValue)
```

### Interpolate values in strings (template literals)

```javascript
`Some string with ${someValue} in it`;
```

### Get the value of a nested property without validating each reference (optional chaining operator)

```javascript
someObject.anotherObject?.someProperty;
```

### Use a default value of a nested property when a reference isn't found (nullish coalescing operator)

```javascript
someObject.anotherObject?.someProperty ?? someFallbackValue;
```

### Extract data from an object / array (destructuring)

```javascript
const { someKey } = someObject;
const [firstKey] = someArray;
```

### Extract data from an object / array with remaining items grouped (rest)

```javascript
const { someKey, ...otherKeys } = someObject;
const [firstKey, ...otherKeys] = someArray;
```

### Expand data from an object / array (spread)

```javascript
{ someKey: someValue, ...someObject }
[ someValue, ...someArray ]
```

### Clone an array

```javascript
[...someArray];
```

### Get whether an array contains a value

```javascript
someArray.includes(someValue);
```

### Divide a string from a separator pattern

```javascript
someString.split(somePattern);
```

### Combine strings with a separator pattern

```javascript
someString.join(somePattern);
```

### Replace patterns in a string

```javascript
someString.replace(somePattern, someReplacement);
```

### Get a character at a specific index in a string

```javascript
someString.charAt(someIndex);
```

### Convert the casing of a string

```javascript
someString.toUpperCase / toLowerCase();
```

### Create asynchronous functions that return a promise

```javascript
async function someFunction() { ... }
```

### Use asynchronous functions that return a promise

```javascript
await someFunction();
```

Examples:

```javascript
const response = await fetch(someUrl);
const json = await res.json();
```

### Sort an array of strings alphabetically

```javascript
someStrings.sort();
```

### Remove falsy values from an array

```javascript
someArray.filter(Boolean);
```

### Get the unique items in an array

```javascript
[...new Set(someArray)];
```

### Get a portion of an array by index

```javascript
someArray.slice(someStartIndex, someEndIndex);
```

### Combine arrays

```javascript
someArray.concat(anotherArray);
```

### Sort an array of numbers by value

```javascript
someNumbers.sort((a, b) => a - b);
```

### Get the largest / smallest number from an array of numbers

```javascript
Math.max / min(...someNumbers);
```

### Convert an array-like iterable object into an array

```javascript
Array.from(someArrayLikeObject);
```

### Get all the keys from an object

```javascript
Object.keys(someObject);
```

### Get all results of matching a string against a regular expression

```javascript
someString.matchAll(someRegularExpression);
```

### Get the global object for an environment (window, global etc.)

```javascript
globalThis;
```

### Use a numbers higher than `Number.MAX_SAFE_INTEGER` / `2^53 - 1` (`BigInt`)

```javascript
const x = 9007199254740991n;
// or
const y = BigInt("9007199254740991234");
```

## Use native browser functionality

Web APIs.

### Send HTTP requests

```javascript
await fetch(someUrl);
```

### Use HTTP request options

```javascript
await fetch(someUrl, {
  method: 'GET/POST/PUT/DELETE',
  headers: {
    'Content-Type': 'application/json',
    'Authorization': Basic some-secret
  }
});
```

Common methods include:

- `GET` to retrieve data
- `POST` to submit data
- `PUT` to replace data
- `DELETE` to remove data

### Use HTTP request responses

```javascript
const response = await fetch(someUrl);
if (status === some-status-code) {
  ...
}
```

Status codes:

- Informational responses (100–199)
- Successful responses (200–299)
- Redirects (300–399)
- Client errors (400–499)
- Server errors (500–599)

Common ones like:

- `200` is a catchall for success
- `404` means the requested resource is not available
- `403` means the requester does not have permission to access the requested resource
- `500` is a catchall for server-side errors

### Get JSON data from a JSON service

```javascript
const response = await fetch(someUrl);
const json = await response.json();
```

### Get the first matching element in the current DOM

```javascript
const element = document.querySelector("some-selector");
```

### Get all matching elements in the current DOM

```javascript
const nodeList = document.querySelectorAll("some-selector");
```

### Reload the current URL

```javascript
location.reload();
```

### Redirect to another URL

```javascript
location.href = someUrl;
```
