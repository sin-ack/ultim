use std::error::Error;

use actix_web::{http::Method, HttpResponse, Responder};
use diesel::{associations::HasTable, Expression, QueryDsl, SelectableExpression, Table};
use ultim::Request;

pub enum PutPartial {
    Yes,
    No,
}

pub trait View {
    fn dispatch(&self, request: &Request) -> Result<impl Responder, Box<dyn Error>> {
        match request.method() {
            &Method::GET => self.get(request),
            &Method::POST => self.post(request),
            &Method::PUT => self.put(request, PutPartial::Yes),
            &Method::PATCH => self.put(request, PutPartial::No),
            &Method::DELETE => self.delete(request),
            _ => Ok(HttpResponse::MethodNotAllowed().into()),
        }
    }

    fn get(&self, request: &Request) -> Result<HttpResponse, Box<dyn Error>> {
        Ok(HttpResponse::MethodNotAllowed().into())
    }

    fn post(&self, request: &Request) -> Result<HttpResponse, Box<dyn Error>> {
        Ok(HttpResponse::MethodNotAllowed().into())
    }

    fn put(&self, request: &Request, partial: PutPartial) -> Result<HttpResponse, Box<dyn Error>> {
        Ok(HttpResponse::MethodNotAllowed().into())
    }

    fn delete(&self, request: &Request) -> Result<HttpResponse, Box<dyn Error>> {
        Ok(HttpResponse::MethodNotAllowed().into())
    }
}

pub trait Queryable<T>
where
    T: Table + HasTable<Table = T>,
{
    type Query: QueryDsl;
    type Selection: SelectableExpression<T>;

    fn get_query(&self) -> Self::Query;
}

impl<T, U> Queryable<T> for U
where
    T: Table + HasTable<Table = T>,
{
    type Query = T;
    type Selection = T::AllColumns;

    fn get_query(&self) -> Self::Query {
        T::table()
    }
}

pub trait Filter<T: QueryDsl> {
    fn filter(&self, query: T) -> T;
}

pub trait ListView<T>: View + Queryable<T>
where
    T: Table + HasTable<Table = T>,
{
    #[allow(unused_variables)]
    fn get_fields(&self, request: &Request) -> Self::Selection;
    fn get_filters(&self, request: &Request) -> Vec<Box<dyn Filter<Self::Query>>>;
    fn get_order(&self, request: &Request) -> Box<dyn Expression<SqlType = T::SqlType>>;

    fn get(&self, request: &Request) -> Result<impl Responder, Box<dyn Error>> {
        let mut query = self.get_query();
        for filter in self.get_filters(request) {
            query = filter.filter(query);
        }

        let query = query.select(self.get_fields(request));
        Ok(HttpResponse::Ok().json(query))
    }
}
