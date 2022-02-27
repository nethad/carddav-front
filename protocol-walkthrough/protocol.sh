#!/usr/bin/env bash

echo "PROPFIND /carddav"
echo ""
curl -X PROPFIND -H "Content-Type: application/xml; charset=utf-8" -H "Depth: 0" -d@current-user-principal.xml http://localhost:8000/carddav
echo ""
echo ""
echo "---"
echo "PROPFIND /carddav/principals/..."
echo ""
curl -X PROPFIND -H "Content-Type: application/xml; charset=utf-8" -H "Depth: 0" -d@addressbook-home-set.xml http://localhost:8000/carddav/principals/users/user@example.org/
echo ""
echo ""
echo "---"
echo "PROPFIND /carddav/addressbooks/... (data support)"
echo ""
curl -X PROPFIND -H "Content-Type: application/xml; charset=utf-8" -H "Depth: 1" -d@addressbooks-data-support.xml http://localhost:8000/carddav/addressbooks/users/user@example.org/
echo ""
echo ""
echo "---"
echo "PROPFIND /carddav/addressbooks/..."
echo ""
curl -X PROPFIND -H "Content-Type: application/xml; charset=utf-8" -H "Depth: 1" -d@addressbooks-props.xml http://localhost:8000/carddav/addressbooks/users/user@example.org/
echo ""
echo ""
echo "---"
echo "REPORT /carddav/addressbooks/.../contacts"
echo ""
curl -X REPORT -H "Content-Type: application/xml; charset=utf-8" -H "Depth: 1" -v -d@contacts-address-data.xml http://localhost:8000/carddav/addressbooks/users/user@example.org/contacts
echo ""

