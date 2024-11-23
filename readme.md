# Video Monitoring

Video monitoring will need to filter for when particular actions are taken. The screen-capture will be taken and pushed to a file store, and then the events will also be pushed to Kafka. So then a processor will load the event with the event time, and the video capture, with the capture time. The screen capture will be filtered to only include the time of the action/directly after the action. 

When the machine learning algorithm predicts the next step, it will predict a json payload that contains the same as the action. For example, it contains a location for a mouse click, or simply a keyboard key. This way, the whole screen capture doesn't need to be kept. Only pictures for the effects of the action, along with the actions themselves.

Then, the machine learning algorithm will input the set of pictures (like a video context, along with the previous actions), and it will predict the next action. This can use a multimodal modal and fine tune to accept the pictures and the actions as text. 

The screen controller will be an interpolation to locations when a click happens, it will say to click somewhere, and the controller will simply move from this location to that. No need to spend the time learning how to move there at first. Predict the actions instead.

Not for user for illegal!!! Was for AI but no time
