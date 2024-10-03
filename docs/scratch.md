# Structure
- Main library is entirely agnostic to the ui
- The `ui` module has the various ui libraries
  - debug is a simple state inspector
  - `doxy` is a single graphic UI using some library
  - `api` is an api to manage state
  - Others may be added in the future for a web interface or whatever
- The `main` file just launches whatever UI is requested

# Later
- Break this into multiple libraries, with this one being not a binary application