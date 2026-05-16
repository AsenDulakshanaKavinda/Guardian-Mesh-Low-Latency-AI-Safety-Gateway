
#[allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;
    let temporary_id_update = "0ef47597-a404-4c5c-b3cd-5c5630cc5950";
    let temporary_id_delete = "dd4eff37-2b47-44e7-9b2b-3a2e8b690e7f";
    let n = 14;

    // 1. CREATE USER
    let create_user_success = hc.do_post(
        "/user/create", 
        json!({
            "username": format!("demo{}", n),
            "email": format!("demo{}gmail.com", n),
            "password": format!("demo{}", n),
        })
    ).await;

    match create_user_success {
        Ok(res) => {
            let status = res.status();
            println!("--> Create! HTTP Status Code: {}", status);
            // assert_eq!(status, 201, "Expected 201 CREATED"); // Adjusted to match your handler's StatusCode::CREATED
            res.print().await?;
        }
        Err(err) => {
            println!("--> Error during creation: {:?}", err);
            return Err(err.into());
        }
    }
    

    // 2. FETCH USER
    let fetch_user_success = hc.do_post(
        "/user/fetch",
        json!({
            "email": "demo7gmail.com"
        })
    ).await;

    match fetch_user_success {
        Ok(res) => {
            let status = res.status();
            println!("--> Fetch! HTTP Status Code: {}", status);
            // assert_eq!(status, 200, "Expected 200 OK from fetch"); 
            res.print().await?; 
        }
        Err(err) => {
            println!("--> Error during fetch: {:?}", err);
            return Err(err.into());
        }
    }
    

    // 3. UPDATE USER (Switched to DO_PUT)
    let update_user_success = hc.do_put(
        &format!("/user/update/{}", temporary_id_update),
        json!({
            "username": "demoupdated02",
        })
    ).await;

    match update_user_success {
        Ok(res) => {
            let status = res.status();
            println!("--> Update! HTTP Status Code: {}", status);
            res.print().await?; 
        }
        Err(err) => {
            println!("--> Error during update: {:?}", err);
            return Err(err.into());
        }
    }


    // 4. DELETE USER (Switched to DO_DELETE and removed unnecessary Json payload)
    let delete_user_success = hc.do_delete(
        &format!("/user/delete/{}", temporary_id_delete)
    ).await;

    match delete_user_success {
        Ok(res) => {
            let status = res.status();
            println!("--> Delete! HTTP Status Code: {}", status);
            // assert_eq!(status, 202, "Expected 202 ACCEPTED"); // Matches StatusCode::ACCEPTED
            res.print().await?; 
        }
        Err(err) => {
            println!("--> Error during delete: {:?}", err);
            return Err(err.into());
        }
    }

    Ok(())
}
    