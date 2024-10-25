# Cookbook Application

The Cookbook Application is a web app designed to manage recipes and ingredients seamlessly. Built with Rust on the backend and TypeScript on the frontend, it allows users to add, update, and delete ingredients while creating recipes with ease. This project showcases efficient data management, API design, and a dynamic user interface.

## Features

- **Ingredient Management**: Add, update, and delete ingredients.
- **Recipe Management**: Create, update, and delete recipes, with associated ingredient quantities.
- **Dynamic UI**: Interactive frontend with custom web components for a smooth user experience.

## Tech Stack

- **Backend**: Rust, Warp, MySQL, `sqlx`
- **Frontend**: TypeScript, HTML, CSS
- **Database**: MySQL

## Getting Started

### Prerequisites

- **Rust**
- **Node.js**
- **MySQL** database

## Dev Test

```sh
# Test for model
cargo watch -q -c -w src/ -x 'test model_db_ -- --test-threads=1 --nocapture'

# Test for web
cargo watch -q -c -w src/ -x 'test web_ -- --test-threads=1 --nocapture'
```


## Dev Web
```sh
cargo watch -q -c -w src/ -x 'run -- ../frontend/web-folder'
```
