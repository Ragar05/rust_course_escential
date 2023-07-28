#[allow(dead_code)]
#[derive(Debug)]
enum UserRoles {
    BASIC = 1,
    ADMIN = 2,
    SUPERADMIN = 3,
}

#[derive(Debug)]
enum Website {
    URL(String),
    INSTAGRAM(String),
    LINKEDIN(String),
    FACEBOOK(String),
}

//Se encuentra en la libreria estandart
// enum Option<T> {
//     Some(T),
//     None
// }

struct User {
    fullname: String,
    email: String,
    age: i32,
    is_active: bool,
    role: UserRoles,
    website: Website,
    cargo: Option<String>,
}

impl User {
    fn from(
        fullname: String,
        email: String,
        age: i32,
        role: UserRoles,
        website: Website,
        cargo: Option<String>,
    ) -> User {
        return User {
            fullname,
            email,
            age,
            role,
            website,
            cargo,
            is_active: true,
        };
    }

    fn get_cargo(&self) -> String {
        if self.cargo.is_none() {
            return String::from("N/A");
        } else {
            return self.cargo.clone().unwrap();
        }

        // match cargo {
        //     Option::Some(cargo) => cargo,
        //     Option::None => String::from("N/A"),
        // },
    }

    fn has_access(&self) -> bool {
        return match self.role {
            UserRoles::ADMIN => true,
            UserRoles::SUPERADMIN => true,
            UserRoles::BASIC => false,
        };
    }
}

fn main() {
    println!("Hello, world!");
    let user = User::from(
        String::from("Ramiro Andres Garcia Urueta"),
        String::from("ramirogarciaurueta1005@gmail.com"),
        32,
        UserRoles::BASIC,
        Website::FACEBOOK(String::from("https://facebook.com")),
        Option::Some(String::from("desarrollador")), // Option::None
    );

    println!(
        "
        Nombre: {}
        Correo: {}
        Edad: {}
        Activo: {}
        Role: {:?}
        Website {:?}
        cargo: {}
        Tiene Acceso: {}",
        user.fullname,
        user.email,
        user.age,
        user.is_active,
        user.role,
        user.website,
        user.get_cargo(),
        user.has_access()
    )
}
