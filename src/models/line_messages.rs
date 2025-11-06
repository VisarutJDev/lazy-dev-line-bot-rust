use serde_json::json;

pub fn get_portfolio_message() -> serde_json::Value {
    json!({
        "type": "flex",
        "altText": "This is a Flex Message",
        "contents": {
          "type": "bubble",
          "hero": {
            "type": "image",
            "url": "https://cdn-images-1.medium.com/v2/resize:fit:400/0*h7JVn-h5hL-mHuH0.jpg",
            "size": "full",
            "aspectRatio": "20:13",
            "aspectMode": "cover",
            "action": {
              "type": "uri",
              "uri": "https://immense-explicitly-mullet.ngrok-free.app//",
            },
          },
          "body": {
            "type": "box",
            "layout": "vertical",
            "spacing": "md",
            "action": {
              "type": "uri",
              "uri": "https://immense-explicitly-mullet.ngrok-free.app//",
            },
            "contents": [
              {
                "type": "text",
                "text": "Portfolio",
                "size": "xl",
                "weight": "bold",
              },
              {
                "type": "box",
                "layout": "vertical",
                "spacing": "sm",
                "contents": [
                  {
                    "type": "box",
                    "layout": "baseline",
                    "contents": [
                      {
                        "type": "text",
                        "text": "คลิกเพื่อเปิด portfolio เพื่อดูประวัติของเรา.",
                        "size": "sm",
                        "align": "start",
                        "color": "#aaaaaa",
                        "wrap": true,
                      },
                    ],
                  },
                ],
              },
            ],
          },
          "footer": {
            "type": "box",
            "layout": "vertical",
            "contents": [
              {
                "type": "button",
                "style": "primary",
                "color": "#905c44",
                "margin": "xxl",
                "action": {
                  "type": "uri",
                  "label": "Open",
                  "uri": "https://immense-explicitly-mullet.ngrok-free.app//",
                },
              },
            ],
          },
        }
    })
}

pub fn get_resume_cv_message() -> serde_json::Value {
    json!({
        "type": "flex",
        "altText": "This is a Flex Message",
        "contents":{
      "type": "carousel",
      "contents": [
        {
          "type": "bubble",
          "body": {
            "type": "box",
            "layout": "vertical",
            "contents": [
              {
                "type": "image",
                "url": "https://cdn-images-1.medium.com/v2/resize:fit:400/0*h7JVn-h5hL-mHuH0.jpg",
                "size": "full",
                "aspectMode": "cover",
                "aspectRatio": "2:3",
                "gravity": "top",
              },
              {
                "type": "box",
                "layout": "vertical",
                "contents": [
                  {
                    "type": "box",
                    "layout": "vertical",
                    "contents": [
                      {
                        "type": "filler",
                      },
                      {
                        "type": "box",
                        "layout": "baseline",
                        "contents": [
                          {
                            "type": "filler",
                          },
                          {
                            "type": "icon",
                            "url": "https://scdn.line-apps.com/n/channel_devcenter/img/flexsnapshot/clip/clip14.png",
                          },
                          {
                            "type": "text",
                            "text": "Resume",
                            "color": "#ffffff",
                            "flex": 0,
                            "offsetTop": "-2px",
                          },
                          {
                            "type": "filler",
                          },
                        ],
                        "spacing": "sm",
                        "action": {
                          "type": "uri",
                          "label": "action",
                          "uri": "https://drive.google.com/file/d/1Jm2gsCdca4yPQj2YIkMtZ-E1OPbhEoi5/view?usp=sharing",
                        },
                      },
                      {
                        "type": "filler",
                      },
                    ],
                    "borderWidth": "1px",
                    "cornerRadius": "4px",
                    "spacing": "sm",
                    "borderColor": "#ffffff",
                    "margin": "xxl",
                    "height": "40px",
                  },
                ],
                "position": "absolute",
                "offsetBottom": "0px",
                "offsetStart": "0px",
                "offsetEnd": "0px",
                "backgroundColor": "#03303Acc",
                "paddingAll": "20px",
                "paddingTop": "18px",
              },
              {
                "type": "box",
                "layout": "vertical",
                "contents": [
                  {
                    "type": "text",
                    "text": "ME",
                    "color": "#ffffff",
                    "align": "center",
                    "size": "xs",
                    "offsetTop": "3px",
                  },
                ],
                "position": "absolute",
                "cornerRadius": "20px",
                "offsetTop": "18px",
                "backgroundColor": "#ff334b",
                "offsetStart": "18px",
                "height": "25px",
                "width": "53px",
              },
            ],
            "paddingAll": "0px",
          },
        },
      ],
    }
    })
}

pub fn get_github_message() -> serde_json::Value {
    json!({
        "type": "flex",
        "altText": "This is a Flex Message",
        "contents": {
      "type": "bubble",
      "body": {
        "type": "box",
        "layout": "vertical",
        "contents": [
          {
            "type": "text",
            "text": "GITHUB",
            "weight": "bold",
            "color": "#1DB446",
            "size": "sm",
          },
          {
            "type": "text",
            "text": "VisarutJDev",
            "weight": "bold",
            "size": "xxl",
            "margin": "md",
          },
          {
            "type": "text",
            "text": "https://github.com/VisarutJDev",
            "size": "xs",
            "color": "#aaaaaa",
            "wrap": true,
          },
          {
            "type": "separator",
            "margin": "xxl",
          },
          {
            "type": "box",
            "layout": "vertical",
            "margin": "xxl",
            "spacing": "sm",
            "contents": [
              {
                "type": "box",
                "layout": "horizontal",
                "contents": [
                  {
                    "type": "text",
                    "text": "algoexport",
                    "size": "sm",
                    "color": "#555555",
                    "flex": 0,
                  },
                  {
                    "type": "text",
                    "text": "Go",
                    "size": "sm",
                    "color": "#111111",
                    "align": "end",
                  },
                ],
              },
              {
                "type": "box",
                "layout": "horizontal",
                "contents": [
                  {
                    "type": "text",
                    "text": "GolangBasicAPI",
                    "size": "sm",
                    "color": "#555555",
                    "flex": 0,
                  },
                  {
                    "type": "text",
                    "text": "Go",
                    "size": "sm",
                    "color": "#111111",
                    "align": "end",
                  },
                ],
              },
              {
                "type": "box",
                "layout": "horizontal",
                "contents": [
                  {
                    "type": "text",
                    "text": "leetcode",
                    "size": "sm",
                    "color": "#555555",
                    "flex": 0,
                  },
                  {
                    "type": "text",
                    "text": "Go",
                    "size": "sm",
                    "color": "#111111",
                    "align": "end",
                  },
                ],
              },
              {
                "type": "box",
                "layout": "horizontal",
                "contents": [
                  {
                    "type": "text",
                    "text": "face-compare",
                    "size": "sm",
                    "color": "#555555",
                    "flex": 0,
                  },
                  {
                    "type": "text",
                    "text": "Go",
                    "size": "sm",
                    "color": "#111111",
                    "align": "end",
                  },
                ],
              },
              {
                "type": "separator",
                "margin": "xxl",
              },
              {
                "type": "box",
                "layout": "horizontal",
                "margin": "xxl",
                "contents": [
                  {
                    "type": "text",
                    "text": "portfolio",
                    "size": "sm",
                    "color": "#555555",
                  },
                  {
                    "type": "text",
                    "text": "React",
                    "size": "sm",
                    "color": "#111111",
                    "align": "end",
                  },
                ],
              },
              {
                "type": "separator",
                "margin": "xxl",
              },
              {
                "type": "box",
                "layout": "horizontal",
                "contents": [
                  {
                    "type": "text",
                    "text": "flutter_basic",
                    "size": "sm",
                    "color": "#555555",
                  },
                  {
                    "type": "text",
                    "text": "flutter",
                    "size": "sm",
                    "color": "#111111",
                    "align": "end",
                  },
                ],
                "margin": "xxl",
              },
            ],
          },
        ],
      },
      "footer": {
        "type": "box",
        "layout": "vertical",
        "contents": [
          {
            "type": "button",
            "action": {
              "type": "uri",
              "label": "Open",
              "uri": "https://github.com/VisarutJDev",
            },
            "margin": "xs",
            "height": "sm",
            "style": "primary",
          },
        ],
      },
      "styles": {
        "footer": {
          "separator": true,
        },
      },
    }
    })
}

pub fn get_blog_carousel_message() -> serde_json::Value {
    json!( {
        "type": "flex",
        "altText": "This is a Flex Message",
        "contents": {
      "type": "carousel",
      "contents": [
        {
          "type": "bubble",
          "size": "hecto",
          "hero": {
            "type": "image",
            "url": "https://cdn-images-1.medium.com/v2/resize:fit:560/1*l6bC6pSj3OjAJXeWdcrgYg.png",
            "size": "full",
            "aspectMode": "cover",
            "aspectRatio": "320:213",
          },
          "body": {
            "type": "box",
            "layout": "vertical",
            "contents": [
              {
                "type": "text",
                "text": "[Docker] Installation and tutorial",
                "weight": "bold",
                "size": "sm",
                "wrap": true,
              },
            ],
            "spacing": "sm",
            "paddingAll": "13px",
          },
          "footer": {
            "type": "box",
            "layout": "vertical",
            "contents": [
              {
                "type": "button",
                "style": "primary",
                "color": "#905c44",
                "margin": "xxl",
                "action": {
                  "type": "uri",
                  "label": "Open",
                  "uri": "https://medium.com/lazy-dev/docker-installation-and-tutorial-cb2060cf6ffa",
                },
              },
            ],
          },
        },
        {
          "type": "bubble",
          "size": "hecto",
          "hero": {
            "type": "image",
            "url": "https://cdn-images-1.medium.com/v2/resize:fit:1024/1*JxG9yFR90CCXyaxpaVMCzw.jpeg",
            "size": "full",
            "aspectMode": "cover",
            "aspectRatio": "320:213",
          },
          "body": {
            "type": "box",
            "layout": "vertical",
            "contents": [
              {
                "type": "text",
                "text": "[Docker] The beginning",
                "weight": "bold",
                "size": "sm",
                "wrap": true,
              },
            ],
            "spacing": "sm",
            "paddingAll": "13px",
          },
          "footer": {
            "type": "box",
            "layout": "vertical",
            "contents": [
              {
                "type": "button",
                "style": "primary",
                "color": "#905c44",
                "margin": "xxl",
                "action": {
                  "type": "uri",
                  "label": "Open",
                  "uri": "https://medium.com/lazy-dev/docker-the-beginning-a738dfd57e09",
                },
              },
            ],
          },
        },
        {
          "type": "bubble",
          "size": "hecto",
          "hero": {
            "type": "image",
            "url": "https://cdn-images-1.medium.com/v2/resize:fit:440/1*K6y9b-I4f92A5Sg66TJiQg.png",
            "size": "full",
            "aspectMode": "cover",
            "aspectRatio": "320:213",
          },
          "body": {
            "type": "box",
            "layout": "vertical",
            "contents": [
              {
                "type": "text",
                "text": "Model and Cost function",
                "weight": "bold",
                "size": "sm",
                "wrap": true,
              },
            ],
            "spacing": "sm",
            "paddingAll": "13px",
          },
          "footer": {
            "type": "box",
            "layout": "vertical",
            "contents": [
              {
                "type": "button",
                "style": "primary",
                "color": "#905c44",
                "margin": "xxl",
                "action": {
                  "type": "uri",
                  "label": "Open",
                  "uri": "https://medium.com/lazy-dev/model-and-cost-function-55929741980c",
                },
              },
            ],
          },
        },
        {
          "type": "bubble",
          "size": "hecto",
          "hero": {
            "type": "image",
            "url": "https://cdn-images-1.medium.com/v2/resize:fit:500/1*UVFzw54ydpbiXt6ucX6Kyw.jpeg",
            "size": "full",
            "aspectMode": "cover",
            "aspectRatio": "320:213",
          },
          "body": {
            "type": "box",
            "layout": "vertical",
            "contents": [
              {
                "type": "text",
                "text": "Introduction to Machine learning",
                "weight": "bold",
                "size": "sm",
                "wrap": true,
              },
            ],
            "spacing": "sm",
            "paddingAll": "13px",
          },
          "footer": {
            "type": "box",
            "layout": "vertical",
            "contents": [
              {
                "type": "button",
                "style": "primary",
                "color": "#905c44",
                "margin": "xxl",
                "action": {
                  "type": "uri",
                  "label": "Open",
                  "uri": "https://medium.com/lazy-dev/introduction-to-machine-learning-ac94e1235752",
                },
              },
            ],
          },
        },
        {
          "type": "bubble",
          "size": "hecto",
          "hero": {
            "type": "image",
            "url": "https://cdn-images-1.medium.com/v2/resize:fit:400/1*aiBD_dyUAg0P0cDBIOy2Jg.png",
            "size": "full",
            "aspectMode": "cover",
            "aspectRatio": "320:213",
          },
          "body": {
            "type": "box",
            "layout": "vertical",
            "contents": [
              {
                "type": "text",
                "text": "[Golang] Let’s custom error like a Gopher",
                "weight": "bold",
                "size": "sm",
                "wrap": true,
              },
            ],
            "spacing": "sm",
            "paddingAll": "13px",
          },
          "footer": {
            "type": "box",
            "layout": "vertical",
            "contents": [
              {
                "type": "button",
                "style": "primary",
                "color": "#905c44",
                "margin": "xxl",
                "action": {
                  "type": "uri",
                  "label": "Open",
                  "uri": "https://medium.com/lazy-dev/golang-lets-custom-error-like-a-gopher-153087d79ba4",
                },
              },
            ],
          },
        },
        {
          "type": "bubble",
          "size": "hecto",
          "hero": {
            "type": "image",
            "url": "https://cdn-images-1.medium.com/v2/resize:fit:220/0*djeeIsHsgqlstyZe.png",
            "size": "full",
            "aspectMode": "cover",
            "aspectRatio": "320:213",
          },
          "body": {
            "type": "box",
            "layout": "vertical",
            "contents": [
              {
                "type": "text",
                "text": "[Rust] Get Started",
                "weight": "bold",
                "size": "sm",
                "wrap": true,
              },
            ],
            "spacing": "sm",
            "paddingAll": "13px",
          },
          "footer": {
            "type": "box",
            "layout": "vertical",
            "contents": [
              {
                "type": "button",
                "style": "primary",
                "color": "#905c44",
                "margin": "xxl",
                "action": {
                  "type": "uri",
                  "label": "Open",
                  "uri": "https://medium.com/lazy-dev/rust-get-started-fdb8916cc401",
                },
              },
            ],
          },
        },
        {
          "type": "bubble",
          "size": "hecto",
          "hero": {
            "type": "image",
            "url": "https://cdn-images-1.medium.com/v2/resize:fit:400/0*h7JVn-h5hL-mHuH0.jpg",
            "size": "full",
            "aspectMode": "cover",
            "aspectRatio": "320:213",
          },
          "body": {
            "type": "box",
            "layout": "vertical",
            "contents": [
              {
                "type": "text",
                "text": "[Golang x OpenCV] Face comparison with GoCV",
                "weight": "bold",
                "size": "sm",
                "wrap": true,
              },
            ],
            "spacing": "sm",
            "paddingAll": "13px",
          },
          "footer": {
            "type": "box",
            "layout": "vertical",
            "contents": [
              {
                "type": "button",
                "style": "primary",
                "color": "#905c44",
                "margin": "xxl",
                "action": {
                  "type": "uri",
                  "label": "Open",
                  "uri": "https://medium.com/lazy-dev/golang-x-opencv-face-comparison-with-gocv-8c160d6c7f1d",
                },
              },
            ],
          },
        },
        {
          "type": "bubble",
          "size": "hecto",
          "hero": {
            "type": "image",
            "url": "https://cdn-images-1.medium.com/v2/resize:fit:400/1*Hoje3fqfZko7i0chLs0BaQ.jpeg",
            "size": "full",
            "aspectMode": "cover",
            "aspectRatio": "320:213",
          },
          "body": {
            "type": "box",
            "layout": "vertical",
            "contents": [
              {
                "type": "text",
                "text": "[Golang x OpenCV] Get start to learn GoCV",
                "weight": "bold",
                "size": "sm",
                "wrap": true,
              },
            ],
            "spacing": "sm",
            "paddingAll": "13px",
          },
          "footer": {
            "type": "box",
            "layout": "vertical",
            "contents": [
              {
                "type": "button",
                "style": "primary",
                "color": "#905c44",
                "margin": "xxl",
                "action": {
                  "type": "uri",
                  "label": "Open",
                  "uri": "https://medium.com/lazy-dev/golang-x-opencv-get-start-to-learn-gocv-9a5e52ab60fe",
                },
              },
            ],
          },
        },
        {
          "type": "bubble",
          "size": "hecto",
          "hero": {
            "type": "image",
            "url": "https://cdn-images-1.medium.com/v2/resize:fit:1024/1*Ce0gUe0LbnhL7ebnDGTp5w.png",
            "size": "full",
            "aspectMode": "cover",
            "aspectRatio": "320:213",
          },
          "body": {
            "type": "box",
            "layout": "vertical",
            "contents": [
              {
                "type": "text",
                "text": "[MongoDB] Let’s learn about MongoDB",
                "weight": "bold",
                "size": "sm",
                "wrap": true,
              },
            ],
            "spacing": "sm",
            "paddingAll": "13px",
          },
          "footer": {
            "type": "box",
            "layout": "vertical",
            "contents": [
              {
                "type": "button",
                "style": "primary",
                "color": "#905c44",
                "margin": "xxl",
                "action": {
                  "type": "uri",
                  "label": "Open",
                  "uri": "https://medium.com/lazy-dev/mongodb-lets-learn-about-mongodb-13439842506e",
                },
              },
            ],
          },
        },
        {
          "type": "bubble",
          "size": "hecto",
          "hero": {
            "type": "image",
            "url": "https://cdn-images-1.medium.com/v2/resize:fit:1024/1*Ce0gUe0LbnhL7ebnDGTp5w.png",
            "size": "full",
            "aspectMode": "cover",
            "aspectRatio": "320:213",
          },
          "body": {
            "type": "box",
            "layout": "vertical",
            "contents": [
              {
                "type": "text",
                "text": "[MongoDB] Installation on Ubuntu",
                "weight": "bold",
                "size": "sm",
                "wrap": true,
              },
            ],
            "spacing": "sm",
            "paddingAll": "13px",
          },
          "footer": {
            "type": "box",
            "layout": "vertical",
            "contents": [
              {
                "type": "button",
                "style": "primary",
                "color": "#905c44",
                "margin": "xxl",
                "action": {
                  "type": "uri",
                  "label": "Open",
                  "uri": "https://medium.com/lazy-dev/mongodb-installation-on-ubuntu-20d738cfd600",
                },
              },
            ],
          },
        },
      ],
    }
    })
}

pub fn get_simple_message() -> serde_json::Value {
    json!({
        "type": "text",
        "text": "Hello! This is a simple text message. You can customize this message to whatever you'd like!"
    })
}
