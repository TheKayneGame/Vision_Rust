pkg load image;
clear;

image = imread("auto2.jpg");

sz = uint32(size(image));

mask = rgb2hsv(image) * 256;
mask = (mask(:,:,1) > 30) & (mask(:,:,1) < 50) & (mask(:,:,2) > 170) & (mask(:,:,3) > 150);
se = strel("disk", 20, 0);
mask = imdilate(mask, se);
mask = imerode(mask, se);
mask = imerode(mask, se);
mask = imdilate(mask, se);

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

image = imcrop(image, [xlow ylow (xhigh - xlow) (yhigh - ylow)]);

image = im2bw(image);
image = imcomplement(image);
se = strel("disk", 4, 0);

sz = size(image);

mask = imread("LetterMasks/3.png");
mask = im2bw(mask);
mask = imcomplement(mask);
mask = imerode(mask, strel("disk", 1, 0));

detect = imerode(image, mask);
detect = imerode(detect, strel("disk", 1, 0));

subplot(2,2,1); imshow(image);
subplot(2,2,2); imshow(mask);
subplot(2,2,3); imshow(detect);