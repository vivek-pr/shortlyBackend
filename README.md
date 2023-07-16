# shortlyBackend
**The main function of this service is to take a long URL and return a much shorter URL that is easier to share and remember. **
**Process:**

1. The user enters a URL on the web frontend.
2. The web frontend sends the URL to the backend service.
3. The backend service checks the database to see if the URL has already been shortened. If it has, the existing short URL is returned. If not, a new short URL is generated.
4. To generate a short URL, the service could take several approaches, such as hashing the long URL and using part of the hash as the short URL, or simply generating a new unique identifier.
5. The service then saves a mapping from the new short URL to the long URL in the database.
6. The short URL is returned to the frontend and displayed to the user.

**URL Redirection:**

When a user visits a short URL, the following steps are followed:

1. The frontend sends a request to the backend with the short URL.
2. The backend looks up the long URL associated with the short URL in the database.
3. The backend then sends a HTTP redirect response to the frontend, with the long URL.
4. The frontend follows the redirect and loads the long URL.
