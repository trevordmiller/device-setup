# HTTP reference

_My reference sheet for communication with HTTP._

## Retrieve data

`GET` request method

## Submit data

`POST` request method

## Replace data

`PUT` request method

## Remove data

`DELETE` request method

## Indicate the result of a request in the response

HTTP response status codes

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

## Include additional information in requests or responses

Headers with `Name: value` like:

```http
Cache-Control: max-age=600
Authorization: Basic YWxhZGRpbjpvcGVuc2VzYW1l
Age: 24
```
