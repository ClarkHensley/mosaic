# Mosaic Specification

## 1. Overview
### 1.1 Description
Mosaic is, by design, distributed. As such, it presents unique challenges that most modern social medias have not experienced. With no centralized set of server(s), each Mosaic User Account, User Profile, Wall, Hub, and Message must be hosted somewhere that is not necessarily accessible to all users at all times.
Therefore, the database structure and methods for distribution needs to be central to design of Mosaic.

### 1.2 Servers
Every Mosaic instance, be it a client or a multi-user hub, must operate as the same type of server. This should, in theory, allow for versatility in storing information on a client as if it were a persistent hub, which will facilitate storage of user-specific Wall posts, Direct Messages between users, etc. As such, each instance must store a local database.

### 1.2.1 Distributed Storage
Because of the distributed nature of Mosaic, we must determine how to manage duplicated, distributed, and shared storage. Even if each instance has a local database, it doesn't make sense to store each message or foreign user's profile contents in each local database. In general, a goal of cached and distributed copies of user data, especially as stored accross dedicated hubs, will be ideal.

### 1.2.2 Security in Distributed Storage
Though security makes up a larger scope of Mosaic, in this case, it is important to control for potentially bad actors who might control a server running a Mosaic instance. At the cost of runtime, ensuring that only signed data is stored "nonlocally" on hubs or others local servers and is decrypted by that user's public key. As such, each database will also have a private-facing sector for actually writing and updating the related user profile(s), hub settings, etc.

### 1.3 Users
A User is the interface by which a human sees Mosaic. A user has a number of important parameters, such as a private/public key pair, a list of associated hubs, a set of personal info, etc.

### 1.4 Messages
A message might be sent in a Hub channel, in a Direct Message, or, perhaps, on
