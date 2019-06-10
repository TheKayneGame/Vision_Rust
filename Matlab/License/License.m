pkg load image;
clear;

isTest = 0;

targetHeight = 50;

image = imread("nummerbord.jpg");

sz = uint32(size(image));

%load the license plate

mask = rgb2hsv(image) * 256;
mask = (mask(:,:,1) > 30) & (mask(:,:,1) < 50) & (mask(:,:,2) > 170) & (mask(:,:,3) > 150);
se = strel("disk", 10, 0);
mask = imclose(mask, se);
mask = imopen(mask, se);

%find the horizontal sides of the license plate

ylow = sz(1) + 1;
yhigh = -1;

for x = 1 : sz(2)
  for y = 1 : sz(1)
    if (mask(y,x) == 1)
      if y < ylow
       ylow = y; 
      end
      
      if y > yhigh
        yhigh = y;
      end
    end 
  end
end

%find the vertical sides of the license plate

xlow = sz(2) + 1;
xhigh = -1;

for y = 1 : sz(1)
  for x = 1 : sz(2)
    if (mask(y,x) == 1)
      if x < xlow
       xlow = x; 
      end
      
      if x > xhigh
        xhigh = x;
      end
    end 
  end
end

%crop the original image to the license plate

image = imcrop(image, [xlow ylow (xhigh - xlow) (yhigh - ylow)]);
sz = size(image);
image = imresize(image, targetHeight/sz(1));
sz = size(image);

%make the image black and white and invert so the license plate letters are white
image = im2bw(image);
image = imcomplement(image);

image = imclearborder(image);

%clean the black and white image

se = strel("disk", 1, 0);
image = imopen(image, se);
image = imclose(image, se);

image = imclose(image, se);
image = imopen(image, se);

%find all masks and remove . and ..
masksList = dir("LetterMasks");
masksList(1:2) = [];

%get objects
objects = regionprops(image, 'Area', 'BoundingBox');

%remove non characters
objects([objects.Area]<=100) = [];

%license string declaration
licenseString = char([]);

%find all characters in image
for character = objects'
  characterImage = imcrop(image, [character.BoundingBox(1) - 1, 0, character.BoundingBox(3) + 1, sz(1)]);
  
  %reset probability and character
  probability = 0;
  detectedChar = '0';
  
  for imageFile = masksList'
    mask = imread(["LetterMasks/", imageFile.name]);
    mask = im2bw(mask);
    detect = imerode(characterImage, mask);
    detect = imclearborder(detect);
    
    %check probability and set character
    detectProbability = regionprops(detect, "Area");
    
    if [detectProbability.Area] > probability
      probability = detectProbability.Area;
      detectedChar = imageFile.name(1);
    end  

    if isTest == 1 
      subplot(2,2,1); imshow(image); title("Black White license");
      subplot(2,2,2); imshow(mask); title("Current Mask");
      subplot(2,2,3); imshow(detect); title("After Erosion");
      subplot(2,2,4); imshow(characterImage); title("Current Character to detect");
    end
    
  end
  
  %push character to end of string
  licenseString = [licenseString, detectedChar];
end

subplot(1,1,1);
imshow(image);
title(licenseString);
