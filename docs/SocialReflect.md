
# Social Reflect

## Introduction

Alternative facts, hate speech, propaganda, and information pollution are all prevalent on social media.  In today's social media, not all voices are equal.

Influencers and marketers use the redundant fake account to amplify their tweets.
It's not surprising that politicians' trashy tweets receive lakhs of retweets, with more than 90% of retweets coming from the same company or IT cell.

Social media has become an essential component of our communication. Low-value information can cause anxiety, paralysis in decision making, and a loss of perspective and moral values.

Ideas, humour, news, blog links, and positive activism for social change are the distinguishing features of social media platforms such as Twitter. Most active social media platforms are controlled by centralised moderation, and their profit-seeking model makes it impossible to reduce junk because their funding is derived from increased engagement rather than quality maintenance.


## Technical Details of Social Reflect

### KYC of users

KYC is performed on each account using the Schelling game algorithm. Jurors are asked to decide whether the profile is valid or not. The likelihood of being drawn as a juror is proportional to the number of tokens staked. The more tokens they stake, the more likely it is that they will be selected as a juror. Jurors are also chosen at random. This safeguards the system against Sybil's attacks.

### Score Schelling game
You can rate from 1-5, without knowing what others are assigning. If the “mean” of all the product rating is near to your rating then the juror will get incentives, otherwise, juror incentives will be deducted. So, the juror will try to match the score with what others will assign based on information available rather than defecting by any arbitrary rating.

The range of  1 to 10 has a problem because the mean works best without extreme values. So, if someone gives 10, and others give 1, the mean result can get screwed due to the 10 outlier. So the trick is to remove outliers by computing the standard deviation. Remove all values more than one standard deviation away from the mean. Then, we calculate the new mean of the left values (it consists of 68.27% data of the set).


```python
import statistics



def calculate_new_mean(items):
    mean = statistics.mean(items)
    print(mean)
    sd = statistics.stdev(items) 
    print(sd)


    #The values less than one standard deviation away from the mean account for 68.27% of the set
    #So we calculate mean of this 68.27% of data removing outlier

    # New data
    new_items = []
    for x in items:
        if x >= mean - sd and x <= mean + sd:
            new_items.append(x)

    print(new_items)

    new_mean = statistics.mean(new_items)
    print(new_mean)
    print("********************")

items = [-10, 1, 1, 1, 5, 1, 1, 7]
calculate_new_mean(items)
# 0.875
# 4.969550137731641
# [1, 1, 1, 5, 1, 1]
# 1.6666666666666667
# ********************
items2 = [-10, -10, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0]
calculate_new_mean(items2)
# -1.5833333333333333
# 3.941811612428832
# [0, 0, 0, 0, 0, 1, 0, 0, 0, 0]
# 0.1
# ********************
items3 = [-10, -10, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, -9, -8, -7, -6, -5, -4, -3, -2, -1]
calculate_new_mean(items3)
# -3.0476190476190474
# 3.8141341150540375
# [0, 0, 0, 0, 0, 0, 0, 0, 0, -6, -5, -4, -3, -2, -1]
# -1.4
# ********************
```

### Selection of moderators
Each account gets a quality score from 1-10, using the score schelling game. This acts as a weight for seq-phragmen approval voting election. The multi-winner representatives now become moderators for social reflect. They will be in the charge of removing posts that don't meet the guidelines of social reflect. 

### Vouching
A family tree is constructed.
If they know each other, friends and relatives will become guarantors for each other.
Moderators chosen through approval voting have the ability to invalidate the fake vouching by down voting the guarantor.

### Incentive system for posts
There are two types of posts. The first type is you post your original thoughts or original blog links. The second type is sharing others' blog links, posts, or news articles. Both types of posts will earn incentives or cryptocurrency every month based upon your post quality, decided by score schelling game. But the original post will get more incentives than the post written by others.  

### Anonymous Posting
Social reflect will also be a platform for scientific and social activism. So, some activists may want to remain anonymous. So, zero-knowledge proof can be used to create anonymous accounts. 




