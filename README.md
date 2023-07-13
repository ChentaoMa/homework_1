# Request Detectives

## CS 410P Chentao Ma (chentao@pdx.edu)

## 2023/7/13

- Bug 1

1. The `GET` `/questions` method will creat a default question each time.

2. First of all, I opened the http://127.0.0.1:8088/questions. But because I turned on the page automatic translation. So I need to close the translation and refresh the page. Unexpectedly, the question list got longer.

3. I think in the original design, the designer's idea was maybe to fill in a default question when the list is empty. But for a variety of reasons, there are no preconditions in the code now. Therefore, every `GET` `/questions` will create a default question and save it in the list.

4. 1) Refresh http://127.0.0.1:8088/questions. 
   
   2) Repeat the following
   ```
    let res = client.get("http://127.0.0.1:8088/questions").send().await?;
    let search_body = res.text().await?;
    println!("List: {}",search_body);
   ``` 

- Bug 2

1. The `DELETE` `/question?question_id=<id>` method will not delete the question with target ID. It removes all questions that are not the target ID.

2. The problem I found when I tried 4 methods. The ID that existed in the list at that time were 0,1,2. I choose to delete question with ID 2, and the result is 2,1.

3. I don't have any thoughts about that. It's just that the designer got the wrong part about what to delete and what to remain.

4. 

```
client.get("http://127.0.0.1:8088/questions").send().await?;
client.get("http://127.0.0.1:8088/questions").send().await?;
client.get("http://127.0.0.1:8088/questions").send().await?;
client.delete("http://127.0.0.1:8088/question?question_id=2").send().await?;
let res = client.get("http://127.0.0.1:8088/questions").send().await?;
    let search_body = res.text().await?;
    println!("List: {}",search_body);
```

- Bug 3

1. Because the ID is determined when the question is created and does not change. As a result, there may be many identical IDs in the list.

2. From the `Bug 2` I got the result list 2,1 . Then the existence of this problem has become inevitable. Just refresh the web page again and got the 2,1,2. Then DELETE with ID 2 . Got 2,2 

3. Because the ID is obtained by looking up the position of the question in the list. So there's a theoretical possibility of duplication. But according to common sense, this situation does not make sense. One possible solution is to construct a counter. And ID is equal to the current value of the counter.

4. 

```
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
```

- Bug 4

1. If multiple questions with the same ID exist in the list, `GET` `/question?question_id=<id>` method can query only the first question that matches the target ID

2. When the same ID exists in the list, the `GET` and `DELETE` by ID need to be considered. After trying it, I found that the delete by ID did not have a problem. There was a problem getting by ID.

3. There is a reasonable possibility that, in common sense, the ID cannot be repeated. So from the point of view of saving operational costs, the general algorithm will end after finding the target ID. However, because of the presence of multiple identical IDs, this algorithm has problems.

4. First, run the `bug 3 -4` part. 

```
     let res1 = client.get("http://127.0.0.1:8088/question?question_id=4").send().await?;
     let search_body = res1.text().await?;
     println!("Search: {}",search_body);
```