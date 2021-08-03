#[derive(Debug, Serialise, Deserialise)]
struct Recipe {
  id: uuid::Uuid,
  title: String,
  content: String,
  author: User,
  published: bool
}






#[derive(Debug)]
struct User {
  id: uuid::Uuid,
  username: String,
  password: String
}

#[derive(Debug)]
struct UserDbIn {
  username: String,
  hashed_password: String
}