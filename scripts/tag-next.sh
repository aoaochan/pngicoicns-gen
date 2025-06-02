latest=$(git tag --sort=-creatordate | head -n 1)

if [ -z "$latest"]; then
  new="v1.0.0"
else
  major=$(echo $latest | cut -d. -f1 | tr -d 'v')
  minor=$(echo $latest | cut -d. -f2)
  patch=$(echo $latest | cut -d. -f3)
  patch=$((patch + 1))
  new="v${major}.${minor}.${patch}"
fi

echo "🔖 New tag: $new"

git tag $new
git push origin $new