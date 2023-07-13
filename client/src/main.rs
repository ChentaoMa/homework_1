use reqwest::Client;
use serde_json::json;

/// example code
///
/// Create a reqwest client
/// let client = Client::new();
/// Make a GET HTTP request to our backend's /example route
/// let res = client.get("http://localhost:8088/example").send().await?;
///
/// Get the response from backend's data
/// let body = res.text().await?;
/// Print out that response
/// println!("GET Response: {}", body);
///
/// Same as GET, but makes a POST request with appropriate header
/// let res = client
///     .post("http://localhost:8088/example")
///     .header("Content-Type", "application/json")
///     .body("Example Body")
///     .send().await?;
///
/// let body = res.text().await?;
/// println!("POST Response: {}", body);
///
/// You'll use these methods along with DELETE to accomplish your task
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a reqwest client
    let client = Client::new();

   // Your code here!
    //set the body of new question
    let content = "Sample Question".to_string();
    let title = "What is the question?".to_string();
    let tags = Option::from(vec!["dumb".to_string(),"sample".to_string()]);
    let question = json!({
        "title": title,
        "content": content,
        "tags": tags
    });

    //set the body of new question 1
    let content1 = "What, another Sample question?".to_string();
    let title1 = "Another Sample Question".to_string();
    let tags1 = Option::from(vec!["dumb".to_string(),"sample".to_string()]);
    let question1 = json!({
        "title": title1,
        "content": content1,
        "tags": tags1
    });

    //Construct the list of questions I want
    client.post("http://127.0.0.1:8088/question").header("Content-Type", "application/json").json(&question).send().await?;
    client.post("http://127.0.0.1:8088/question").header("Content-Type", "application/json").json(&question).send().await?;
    client.post("http://127.0.0.1:8088/question").header("Content-Type", "application/json").json(&question).send().await?;
    client.post("http://127.0.0.1:8088/question").header("Content-Type", "application/json").json(&question).send().await?;
    client.post("http://127.0.0.1:8088/question").header("Content-Type", "application/json").json(&question).send().await?;
    client.delete("http://127.0.0.1:8088/question?question_id=4").send().await?;
    client.post("http://127.0.0.1:8088/question").header("Content-Type", "application/json").json(&question1).send().await?;
    client.post("http://127.0.0.1:8088/question").header("Content-Type", "application/json").json(&question1).send().await?;
    client.post("http://127.0.0.1:8088/question").header("Content-Type", "application/json").json(&question1).send().await?;
    client.post("http://127.0.0.1:8088/question").header("Content-Type", "application/json").json(&question1).send().await?;
    client.delete("http://127.0.0.1:8088/question?question_id=4").send().await?;

     //The bug : question can have the same ID
     //The bug : The DELETE will delete all questions except the target ID
     let res = client.get("http://127.0.0.1:8088/questions").send().await?;
     let body = res.text().await?;
     println!("List: {}",body);
    
     //The bug :GET will get a default question
     let res1 = client.get("http://127.0.0.1:8088/questions").send().await?;
     let body1 = res1.text().await?;
     println!("List: {}",body1);
     
     //The bug: the SEARCH by ID will only find the first question with the target ID
     let res1 = client.get("http://127.0.0.1:8088/question?question_id=4").send().await?;
     let search_body = res1.text().await?;
     println!("Search: {}",search_body);

     Ok(())
}

