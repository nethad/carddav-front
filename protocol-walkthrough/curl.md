```
curl -X PROPFIND -H "Content-Type: application/xml; charset=utf-8" -H "Depth: 0" -v -d@current-user-principal.xml http://localhost:8000/carddav
```

```
curl -X PROPFIND -H "Content-Type: application/xml; charset=utf-8" -H "Depth: 0" -v -d@addressbook-home-set.xml http://localhost:8000/carddav/principals/users/user@example.org/
```

```
curl -X PROPFIND -H "Content-Type: application/xml; charset=utf-8" -H "Depth: 1" -v -d@addressbooks-props.xml http://localhost:8000/carddav/addressbooks/users/user@example.org/
```
