#[macro_export]
macro_rules! build_model {
    ( $struct_name:ident; $entity_struct_name:ident; $table_id:ident; $table:expr => {
        $( $attr_name:ident : $attr_type:ty ),*
    }) => {
        #[table_name=$table]
        #[derive(Serialize, Insertable, Debug, Clone)]
        pub struct $struct_name {
            $( pub $attr_name : $attr_type ),*
        }

        #[table_name=$table]
        #[derive(Queryable, Serialize, Debug, Clone, AsChangeset)]
        pub struct $entity_struct_name {
            pub id: i32,
            $( pub $attr_name : $attr_type ),*
        }

        impl $struct_name {
            pub fn from_entity(entity: $entity_struct_name) -> (i32, $struct_name) {
                (entity.id,
                 $struct_name {
                    $( $attr_name : entity.$attr_name ),*
                 })
            }

        }

        impl $entity_struct_name {
            pub fn from(entity_id: i32, base_struct: &$struct_name) -> $entity_struct_name {
                $entity_struct_name {
                    id : entity_id,
                    $( $attr_name : base_struct.$attr_name.clone() ),*
                }
            }
        }

       impl $struct_name {
            pub fn get(get_id: i32, db: &DbConn) -> QueryResult<(i32, $struct_name)> {
                use crate::schema::$table_id::dsl::*;

                let entity = $table_id
                    .filter(id.eq(get_id))
                    .first::<$entity_struct_name>(&db.0)?;
                Ok($struct_name::from_entity(entity))
            }

            pub fn insert(&self, db: &DbConn) -> QueryResult<(i32,$struct_name)> {
                use crate::schema::$table_id::dsl::*;
                use crate::schema::$table_id::dsl::id;

                diesel::insert_into($table_id)
                    .values(self)
                    .execute(&db.0)?;

                let entity: $entity_struct_name = $table_id
                    .order(id.desc())
                    .first(&db.0)?;

                Ok($struct_name::from_entity(entity))
            }
       }

       impl $struct_name {
            pub fn update( update_id: i32, new_values: &$struct_name,db: &DbConn) -> QueryResult<(i32, $struct_name)>
            {
                use crate::schema::$table_id::dsl::*;
                use crate::schema::$table_id::dsl::id;

                let new_entity = $entity_struct_name::from(update_id, new_values);
                print!("{:?}", &new_entity);
                diesel::update($table_id.filter(id.eq(update_id)))
                    .set(&new_entity)
                    .execute(&db.0)?;

                let updated_entity : $entity_struct_name =
                    $table_id.filter(id.eq(update_id))
                    .first(&db.0)?;

                Ok($struct_name::from_entity(updated_entity))
            }
       }
    }
}
